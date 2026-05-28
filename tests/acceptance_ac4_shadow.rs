//! AC4: Date-specific stamp file shadows the recurring file for the
//! same MM-DD: when both `2026-12-31.json` and `12-31.json` exist,
//! `which 2026-12-31` returns the date-specific one.
//!
//! Read-only after scaffold.

use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_day-stamp"))
        .env("DAY_STAMP_CATALOG_DIR", dir)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn acceptance_ac4_shadow() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    // seed writes the recurring 12-31.json (id "year-ends").
    assert!(run(dir, &["seed"]).status.success());
    // add a date-specific stamp for the same calendar day.
    let add = run(
        dir,
        &[
            "add", "--id", "nye-2026", "--title", "NYE 2026", "--date", "2026-12-31", "--line",
            "Auld lang syne.",
        ],
    );
    assert!(add.status.success(), "add must succeed: {add:?}");

    let out = run(dir, &["which", "2026-12-31"]);
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        "nye-2026",
        "date-specific file must shadow the recurring one"
    );
}
