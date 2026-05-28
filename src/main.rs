//! `day-stamp` — manage and render the special-day stamp catalog.
//!
//! Subcommands: `which`, `list`, `add`, `render`, `seed`. See the PRD
//! (`PRD-daily-receipt-stamps.md`) for the catalog format and the lookup
//! contract consumed by `day-summarize` / `day-haiku`.

#![cfg_attr(not(test), forbid(unsafe_code))]

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use chrono::Local;
use clap::{Args, Parser, Subcommand};

use day_stamps::catalog::{self, Stamp};
use day_stamps::render;
use day_stamps::validate;

/// Exit code for a generic / not-found error.
const EXIT_ERROR: u8 = 3;
/// Exit code for an `add` collision (refuse to overwrite).
const EXIT_COLLISION: u8 = 4;
/// Exit code for a schema violation in a stamp's `lines`.
const EXIT_SCHEMA: u8 = 5;

/// Embedded seed catalog: `(filename, json)` pairs written by `seed`.
const SEEDS: [(&str, &str); 4] = [
    ("01-01.json", include_str!("../seeds/01-01.json")),
    ("12-31.json", include_str!("../seeds/12-31.json")),
    ("2026-05-22.json", include_str!("../seeds/2026-05-22.json")),
    ("2026-05-27.json", include_str!("../seeds/2026-05-27.json")),
];

#[derive(Parser)]
#[command(name = "day-stamp", version, about = "Special-day stamp catalog for daily-receipt")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print which stamp (if any) fires on a date (default: today).
    Which {
        /// Date as `YYYY-MM-DD`; omit for today.
        date: Option<String>,
    },
    /// List all stamps in the catalog.
    List {
        /// Emit a JSON array of full stamp metadata.
        #[arg(long)]
        json: bool,
        /// Only list stamps of this `kind`.
        #[arg(long)]
        kind: Option<String>,
    },
    /// Add a new stamp to the catalog (refuses to overwrite).
    Add(AddArgs),
    /// Render a stamp's ESC/POS bytes to a file. `<id>` may be `today`.
    Render {
        /// Stamp id, or the literal `today`.
        id: String,
        /// Output path for the ESC/POS byte stream.
        #[arg(long)]
        out: PathBuf,
    },
    /// Write the starter catalog (idempotent; never overwrites).
    Seed,
}

/// Fields for `day-stamp add`.
#[derive(Args)]
struct AddArgs {
    /// Stable identifier, e.g. `printer-arrives`.
    #[arg(long)]
    id: String,
    /// Display title.
    #[arg(long)]
    title: String,
    /// Date the stamp fires, as `YYYY-MM-DD`.
    #[arg(long)]
    date: String,
    /// Body line; repeat `--line` for multiple lines.
    #[arg(long = "line")]
    lines: Vec<String>,
    /// Optional secondary line under the title.
    #[arg(long, default_value = "")]
    subtitle: String,
    /// `birthday | anniversary | milestone | named-day | custom`.
    #[arg(long, default_value = "custom")]
    kind: String,
    /// `small | medium | large` (trailing feed-line count).
    #[arg(long, default_value = "medium")]
    size_hint: String,
    /// Free-form user category.
    #[arg(long, default_value = "")]
    category: String,
    /// Optional raster glyph seed for a glyph-only stamp.
    #[arg(long)]
    glyph_seed: Option<u64>,
}

fn line(out: &mut impl Write, s: &str) {
    let _ = writeln!(out, "{s}");
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let dir = match catalog::catalog_dir() {
        Ok(d) => d,
        Err(e) => {
            line(&mut io::stderr(), &format!("day-stamp: {e}"));
            return ExitCode::from(EXIT_ERROR);
        }
    };
    match cli.cmd {
        Cmd::Which { date } => cmd_which(&dir, date.as_deref()),
        Cmd::List { json, kind } => cmd_list(&dir, json, kind.as_deref()),
        Cmd::Add(args) => cmd_add(&dir, &args),
        Cmd::Render { id, out } => cmd_render(&dir, &id, &out),
        Cmd::Seed => cmd_seed(&dir),
    }
}

fn cmd_which(dir: &Path, date: Option<&str>) -> ExitCode {
    let parsed = match date {
        Some(s) => match catalog::parse_date(s) {
            Ok(d) => d,
            Err(e) => {
                line(&mut io::stderr(), &format!("day-stamp: {e}"));
                return ExitCode::from(EXIT_ERROR);
            }
        },
        None => Local::now().date_naive(),
    };
    if let Some(stamp) = catalog::which(dir, parsed) {
        line(&mut io::stdout(), &stamp.id);
    }
    ExitCode::SUCCESS
}

fn cmd_list(dir: &Path, json: bool, kind: Option<&str>) -> ExitCode {
    let mut stamps = match catalog::list_all(dir) {
        Ok(v) => v,
        Err(e) => {
            line(&mut io::stderr(), &format!("day-stamp: {e}"));
            return ExitCode::from(EXIT_ERROR);
        }
    };
    if let Some(k) = kind {
        stamps.retain(|s| s.kind == k);
    }
    if json {
        match serde_json::to_string_pretty(&stamps) {
            Ok(s) => line(&mut io::stdout(), &s),
            Err(e) => {
                line(&mut io::stderr(), &format!("day-stamp: {e}"));
                return ExitCode::from(EXIT_ERROR);
            }
        }
    } else {
        let mut out = io::stdout();
        for s in &stamps {
            line(&mut out, &format!("{}  {}", s.id, s.title));
        }
    }
    ExitCode::SUCCESS
}

fn cmd_add(dir: &Path, args: &AddArgs) -> ExitCode {
    if let Err(e) = catalog::parse_date(&args.date) {
        line(&mut io::stderr(), &format!("day-stamp: invalid --date: {e}"));
        return ExitCode::from(EXIT_ERROR);
    }
    if let Err(le) = validate::validate_lines(&args.lines) {
        line(&mut io::stderr(), &format!("day-stamp: line {}: {}", le.line, le.reason));
        return ExitCode::from(EXIT_SCHEMA);
    }
    let filename = format!("{}.json", args.date);
    if dir.join(&filename).exists() {
        line(
            &mut io::stderr(),
            &format!("day-stamp: refusing to overwrite existing stamp for {}", args.date),
        );
        return ExitCode::from(EXIT_COLLISION);
    }
    let stamp = Stamp {
        id: args.id.clone(),
        kind: args.kind.clone(),
        title: args.title.clone(),
        subtitle: args.subtitle.clone(),
        lines: args.lines.clone(),
        glyph_seed: args.glyph_seed,
        size_hint: args.size_hint.clone(),
        category: args.category.clone(),
        created_by: String::new(),
        created_at: Local::now().to_rfc3339(),
    };
    if let Err(e) = catalog::save_stamp(dir, &filename, &stamp) {
        line(&mut io::stderr(), &format!("day-stamp: {e}"));
        return ExitCode::from(EXIT_ERROR);
    }
    line(&mut io::stdout(), &format!("wrote {filename}"));
    ExitCode::SUCCESS
}

fn cmd_render(dir: &Path, id: &str, out: &Path) -> ExitCode {
    let stamp = if id == "today" {
        if let Some(s) = catalog::which(dir, Local::now().date_naive()) {
            s
        } else {
            line(&mut io::stderr(), "day-stamp: no stamp for today");
            return ExitCode::from(EXIT_ERROR);
        }
    } else {
        match catalog::find_by_id(dir, id) {
            Ok(Some((_, s))) => s,
            Ok(None) => {
                line(&mut io::stderr(), &format!("day-stamp: no stamp with id {id}"));
                return ExitCode::from(EXIT_ERROR);
            }
            Err(e) => {
                line(&mut io::stderr(), &format!("day-stamp: {e}"));
                return ExitCode::from(EXIT_ERROR);
            }
        }
    };
    if let Err(le) = validate::validate_lines(&stamp.lines) {
        line(&mut io::stderr(), &format!("day-stamp: line {}: {}", le.line, le.reason));
        return ExitCode::from(EXIT_SCHEMA);
    }
    let bytes = render::render(&stamp);
    if let Err(e) = std::fs::write(out, &bytes) {
        line(&mut io::stderr(), &format!("day-stamp: {e}"));
        return ExitCode::from(EXIT_ERROR);
    }
    ExitCode::SUCCESS
}

fn cmd_seed(dir: &Path) -> ExitCode {
    if let Err(e) = std::fs::create_dir_all(dir) {
        line(&mut io::stderr(), &format!("day-stamp: {e}"));
        return ExitCode::from(EXIT_ERROR);
    }
    for (name, body) in SEEDS {
        let path = dir.join(name);
        if path.exists() {
            continue;
        }
        if let Err(e) = std::fs::write(&path, body) {
            line(&mut io::stderr(), &format!("day-stamp: {e}"));
            return ExitCode::from(EXIT_ERROR);
        }
    }
    ExitCode::SUCCESS
}
