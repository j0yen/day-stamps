//! AC7: `day-stamp render` with no `lines` and `glyph_seed: 42` renders
//! a 24×24 raster glyph (reuses daily-receipt's renderer via dep, or
//! shells out to `daily-receipt render` with a fake summary — either is
//! acceptable; pick the simpler path).
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

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn acceptance_ac7_glyph_only() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();

    // A glyph-only stamp: no --line, glyph_seed set.
    let add = run(
        dir,
        &[
            "add", "--id", "glyph42", "--title", "Glyph", "--date", "2026-06-01", "--glyph-seed",
            "42",
        ],
    );
    assert!(add.status.success(), "add glyph-only stamp: {add:?}");

    let out_path = dir.join("g.escpos");
    let render = run(dir, &["render", "glyph42", "--out", out_path.to_str().unwrap()]);
    assert!(render.status.success(), "render must exit 0: {render:?}");

    let bytes = std::fs::read(&out_path).unwrap();
    assert_eq!(&bytes[0..2], &[0x1B, 0x40], "begins with ESC @");
    // GS v 0 — raster bit-image command for the 24x24 glyph.
    assert!(contains(&bytes, &[0x1D, 0x76, 0x30]), "emits a raster bit-image");
    // 8-byte raster header + 24 rows * 3 bytes = 72 raster payload bytes.
    assert!(bytes.len() > 72, "raster payload present");
}
