//! AC8: `day-stamp list --json` prints a JSON array of all stamps with
//! full metadata; sortable by `created_at`.
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
fn acceptance_ac8_list_json() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    assert!(run(dir, &["seed"]).status.success());

    let out = run(dir, &["list", "--json"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let arr = v.as_array().expect("list --json is a JSON array");
    assert_eq!(arr.len(), 4, "all four seeded stamps are listed");

    // Full metadata present on each entry.
    for s in arr {
        assert!(s.get("id").is_some());
        assert!(s.get("title").is_some());
        assert!(s.get("created_at").is_some());
        assert!(s.get("kind").is_some());
    }

    // Sorted ascending by created_at.
    let dates: Vec<&str> = arr.iter().map(|s| s["created_at"].as_str().unwrap()).collect();
    let mut sorted = dates.clone();
    sorted.sort_unstable();
    assert_eq!(dates, sorted, "entries are sorted by created_at");
}
