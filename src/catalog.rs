//! Stamp catalog: on-disk model, path resolution, and date lookup.
//!
//! One stamp per JSON file. Date-specific files (`<YYYY-MM-DD>.json`) shadow
//! recurring files (`<MM-DD>.json`) for the same calendar day.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Boxed error alias for IO/serialization failures.
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

/// A single special-day stamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stamp {
    /// Stable identifier, e.g. `printer-arrives`.
    pub id: String,
    /// One of `birthday | anniversary | milestone | named-day | custom`.
    #[serde(default = "default_kind")]
    pub kind: String,
    /// Display title, rendered centered.
    pub title: String,
    /// Secondary line under the title (often the date).
    #[serde(default)]
    pub subtitle: String,
    /// Body lines, 0..=12, each ≤40 graphemes.
    #[serde(default)]
    pub lines: Vec<String>,
    /// Optional glyph seed; when set with empty `lines`, render a raster.
    #[serde(default)]
    pub glyph_seed: Option<u64>,
    /// `small | medium | large`; controls trailing feed-line count.
    #[serde(default = "default_size")]
    pub size_hint: String,
    /// Free-form user category; not interpreted by code.
    #[serde(default)]
    pub category: String,
    /// Who authored the stamp.
    #[serde(default)]
    pub created_by: String,
    /// RFC 3339 creation timestamp; sorts `list --json`.
    #[serde(default)]
    pub created_at: String,
}

fn default_kind() -> String {
    "custom".to_owned()
}
fn default_size() -> String {
    "medium".to_owned()
}

/// Resolve the catalog directory: explicit override, then XDG, then the
/// `~/.config` default, then the `~/.claude` fallback.
///
/// # Errors
/// Returns an error when neither `DAY_STAMP_CATALOG_DIR`, `XDG_CONFIG_HOME`,
/// nor `HOME` is set to a non-empty value.
pub fn catalog_dir() -> Res<PathBuf> {
    if let Ok(dir) = env::var("DAY_STAMP_CATALOG_DIR") {
        if !dir.is_empty() {
            return Ok(PathBuf::from(dir));
        }
    }
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Ok(Path::new(&xdg).join("daily-receipt").join("stamps"));
        }
    }
    if let Ok(home) = env::var("HOME") {
        if !home.is_empty() {
            let cfg = Path::new(&home).join(".config").join("daily-receipt").join("stamps");
            return Ok(cfg);
        }
    }
    Err("cannot resolve catalog dir: set XDG_CONFIG_HOME or HOME".into())
}

/// Parse a `YYYY-MM-DD` string into a [`NaiveDate`].
///
/// # Errors
/// Returns an error when `s` is not a valid ISO date.
pub fn parse_date(s: &str) -> Res<NaiveDate> {
    Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)
}

/// Load and deserialize a stamp file.
///
/// # Errors
/// Returns an error on IO failure or malformed JSON.
pub fn load_stamp(path: &Path) -> Res<Stamp> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

/// Atomically write a stamp to `dir/<filename>` via tempfile + rename.
///
/// # Errors
/// Returns an error on IO or serialization failure.
pub fn save_stamp(dir: &Path, filename: &str, stamp: &Stamp) -> Res<()> {
    fs::create_dir_all(dir)?;
    let body = serde_json::to_string_pretty(stamp)?;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let tmp = dir.join(format!(".tmp.{}.{nanos}", std::process::id()));
    fs::write(&tmp, body.as_bytes())?;
    fs::rename(&tmp, dir.join(filename))?;
    Ok(())
}

/// Return the stamp matching `date`: date-specific file wins over recurring.
#[must_use]
pub fn which(dir: &Path, date: NaiveDate) -> Option<Stamp> {
    let ymd = date.format("%Y-%m-%d").to_string();
    let mmdd = date.format("%m-%d").to_string();
    for name in [format!("{ymd}.json"), format!("{mmdd}.json")] {
        let path = dir.join(&name);
        if path.is_file() {
            if let Ok(stamp) = load_stamp(&path) {
                return Some(stamp);
            }
        }
    }
    None
}

/// Find the file whose stamp `id` matches; unparseable files are skipped.
///
/// # Errors
/// Returns an error if the catalog directory exists but cannot be read.
pub fn find_by_id(dir: &Path, id: &str) -> Res<Option<(PathBuf, Stamp)>> {
    if !dir.is_dir() {
        return Ok(None);
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension() != Some(OsStr::new("json")) {
            continue;
        }
        if let Ok(stamp) = load_stamp(&path) {
            if stamp.id == id {
                return Ok(Some((path, stamp)));
            }
        }
    }
    Ok(None)
}

/// List all stamps in the catalog, sorted by `created_at`.
///
/// # Errors
/// Returns an error if the directory cannot be read.
pub fn list_all(dir: &Path) -> Res<Vec<Stamp>> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension() != Some(OsStr::new("json")) {
            continue;
        }
        if let Ok(stamp) = load_stamp(&path) {
            out.push(stamp);
        }
    }
    out.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(out)
}
