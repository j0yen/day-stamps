//! AC3: `day-stamp which 2026-01-01` matches the recurring `01-01.json`
//! stamp. Verified for at least two distinct years (2026, 2030) in the
//! same test.
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
fn acceptance_ac3_recurring() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    assert!(run(dir, &["seed"]).status.success());

    for year in ["2026-01-01", "2030-01-01"] {
        let out = run(dir, &["which", year]);
        assert!(out.status.success(), "{year} must exit 0");
        assert_eq!(
            String::from_utf8_lossy(&out.stdout).trim(),
            "new-year",
            "recurring 01-01.json must fire for {year}"
        );
    }
}
