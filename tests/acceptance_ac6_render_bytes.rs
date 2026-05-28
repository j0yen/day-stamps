//! AC6: `day-stamp render printer-arrives --out /tmp/p.escpos` writes a
//! non-empty ESC/POS byte stream that begins with ESC '@' (`0x1B 0x40`),
//! contains the stamp title bytes verbatim, ends with feed-and-cut bytes
//! (`0x1D 0x56 0x42 0x00`). Daily-receipt AC2 compatibility.
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
fn acceptance_ac6_render_bytes() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    assert!(run(dir, &["seed"]).status.success());

    let out_path = dir.join("p.escpos");
    let render = run(dir, &["render", "printer-arrives", "--out", out_path.to_str().unwrap()]);
    assert!(render.status.success(), "render must exit 0: {render:?}");

    let bytes = std::fs::read(&out_path).unwrap();
    assert!(!bytes.is_empty(), "byte stream is non-empty");
    assert_eq!(&bytes[0..2], &[0x1B, 0x40], "begins with ESC @");
    assert!(
        contains(&bytes, b"The MASUNG IP1000 lands"),
        "contains the title verbatim"
    );
    let tail = &bytes[bytes.len() - 4..];
    assert_eq!(tail, &[0x1D, 0x56, 0x42, 0x00], "ends with feed-and-cut");
}
