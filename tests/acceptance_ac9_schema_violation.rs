//! AC9: Each stamp file's `lines` violating the schema (≥13 lines, or
//! line >40 graphemes) is rejected at `add`-time with exit 5;
//! `render`-time of an existing-but-malformed file also exits 5 with the
//! offending line numbered in stderr.
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
fn acceptance_ac9_schema_violation() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();

    // --- add-time: a single line wider than 40 graphemes is rejected. ---
    let wide = "x".repeat(41);
    let out = run(
        dir,
        &["add", "--id", "wide", "--title", "T", "--date", "2026-06-01", "--line", &wide],
    );
    assert_eq!(out.status.code(), Some(5), "wide line must exit 5");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("line 1"), "stderr names the offending line: {stderr}");
    assert!(
        !dir.join("2026-06-01.json").exists(),
        "rejected add must not write the file"
    );

    // --- add-time: more than 12 lines is rejected. ---
    let mut args: Vec<&str> = vec!["add", "--id", "tall", "--title", "T", "--date", "2026-06-02"];
    for _ in 0..13 {
        args.push("--line");
        args.push("ok");
    }
    let out = run(dir, &args);
    assert_eq!(out.status.code(), Some(5), "13 lines must exit 5");

    // --- render-time: an existing-but-malformed file also exits 5. ---
    let bad = format!(
        r#"{{"id":"badrender","kind":"custom","title":"T","subtitle":"",
            "lines":["fine","{}"],"glyph_seed":null,"size_hint":"medium",
            "category":"","created_by":"","created_at":"2026-06-03T00:00:00+00:00"}}"#,
        "y".repeat(41)
    );
    std::fs::write(dir.join("2026-06-03.json"), bad).unwrap();
    let outfile = tmp.path().join("out.bin");
    let out = run(
        dir,
        &["render", "badrender", "--out", outfile.to_str().unwrap()],
    );
    assert_eq!(out.status.code(), Some(5), "malformed render must exit 5");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("line 2"),
        "render stderr numbers the offending line (line 2): {stderr}"
    );
    assert!(!outfile.exists(), "rejected render must not write output bytes");
}
