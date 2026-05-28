//! AC1: `day-stamp seed` against an empty
//! `$XDG_CONFIG_HOME/daily-receipt/stamps/` writes 4 starter stamp
//! files. Re-running `seed` does not overwrite; exits 0.
//!
//! Read-only after scaffold: the edit-agent must NOT modify acceptance
//! tests. If a test is wrong, write agent/intent_card_amendment_request.json.

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

fn json_count(dir: &Path) -> usize {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .count()
}

#[test]
fn acceptance_ac1_seed_idempotent() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();

    let out = run(dir, &["seed"]);
    assert!(out.status.success(), "first seed must exit 0");
    assert_eq!(json_count(dir), 4, "seed writes exactly 4 starter files");

    // Mutate one seeded file to prove the second seed does not overwrite.
    let sentinel = dir.join("01-01.json");
    std::fs::write(&sentinel, b"MUTATED").unwrap();

    let out2 = run(dir, &["seed"]);
    assert!(out2.status.success(), "re-seed must exit 0");
    assert_eq!(json_count(dir), 4, "re-seed does not add files");
    assert_eq!(
        std::fs::read(&sentinel).unwrap(),
        b"MUTATED",
        "re-seed must not overwrite an existing file"
    );
}
