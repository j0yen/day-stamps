//! AC2: `day-stamp which 2026-05-27` prints `printer-arrives` to stdout.
//! `which 2026-05-28` (no stamp) prints nothing and exits 0.
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
fn acceptance_ac2_which() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    assert!(run(dir, &["seed"]).status.success());

    let hit = run(dir, &["which", "2026-05-27"]);
    assert!(hit.status.success());
    assert_eq!(String::from_utf8_lossy(&hit.stdout).trim(), "printer-arrives");

    let miss = run(dir, &["which", "2026-05-28"]);
    assert!(miss.status.success(), "a miss still exits 0");
    assert!(miss.stdout.is_empty(), "a miss prints nothing");
}
