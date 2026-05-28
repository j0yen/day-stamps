//! AC5: `day-stamp add --id foo --title T --date 2026-05-30 --line "Hello"`
//! creates `2026-05-30.json` with the expected shape; running again with
//! the same `--id` and date errors out with exit 4 (do not silently
//! overwrite).
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
fn acceptance_ac5_add_collision() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    let args = [
        "add", "--id", "foo", "--title", "T", "--date", "2026-05-30", "--line", "Hello",
    ];

    let first = run(dir, &args);
    assert!(first.status.success(), "first add must exit 0");

    let path = dir.join("2026-05-30.json");
    assert!(path.is_file(), "add creates the date-keyed file");
    let body = std::fs::read_to_string(&path).unwrap();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["id"], "foo");
    assert_eq!(v["title"], "T");
    assert_eq!(v["lines"][0], "Hello");

    let second = run(dir, &args);
    assert_eq!(
        second.status.code(),
        Some(4),
        "re-adding the same id+date must exit 4, not overwrite"
    );
}
