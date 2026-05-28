//! ESC/POS byte assembly for a stamp.
//!
//! Layout: `ESC @` init, then either a 24×24 raster glyph (when `lines` is
//! empty and `glyph_seed` is set) or centered title/subtitle/body text,
//! followed by `size_hint`-many feed lines and a `GS V B 0` cut. The
//! glyph path uses a built-in deterministic raster derived from
//! `glyph_seed` (the simpler of the two AC7 options) so rendering needs no
//! external `daily-receipt` binary.

use crate::catalog::Stamp;

/// `ESC @` — initialize printer.
const INIT: [u8; 2] = [0x1B, 0x40];
/// `ESC a 1` — center justification.
const CENTER: [u8; 3] = [0x1B, 0x61, 0x01];
/// `ESC a 0` — left justification.
const LEFT: [u8; 3] = [0x1B, 0x61, 0x00];
/// `GS V B 0` — feed and full cut.
const CUT: [u8; 4] = [0x1D, 0x56, 0x42, 0x00];

/// Raster dimensions: 24×24 dots → 3 bytes per row, 24 rows.
const RASTER_ROW_BYTES: usize = 3;
const RASTER_ROWS: usize = 24;

fn feed_lines(size_hint: &str) -> usize {
    match size_hint {
        "small" => 2,
        "large" => 8,
        _ => 4,
    }
}

fn push_text_line(buf: &mut Vec<u8>, line: &str) {
    buf.extend_from_slice(line.as_bytes());
    buf.push(b'\n');
}

/// Deterministic 24×24 raster bit-image from `seed` (`GS v 0` command).
fn glyph_raster(seed: u64) -> Vec<u8> {
    let row_bytes = u8::try_from(RASTER_ROW_BYTES).unwrap_or(0);
    let rows = u8::try_from(RASTER_ROWS).unwrap_or(0);
    let mut out = vec![0x1D, 0x76, 0x30, 0x00, row_bytes, 0x00, rows, 0x00];
    let mut state = seed;
    for _ in 0..(RASTER_ROW_BYTES * RASTER_ROWS) {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        out.push(u8::try_from((state >> 24) & 0xFF).unwrap_or(0));
    }
    out
}

/// Assemble the ESC/POS byte stream for `stamp`.
#[must_use]
pub fn render(stamp: &Stamp) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&INIT);
    if stamp.lines.is_empty() {
        if let Some(seed) = stamp.glyph_seed {
            buf.extend_from_slice(&glyph_raster(seed));
        }
    } else {
        buf.extend_from_slice(&CENTER);
        push_text_line(&mut buf, &stamp.title);
        if !stamp.subtitle.is_empty() {
            push_text_line(&mut buf, &stamp.subtitle);
        }
        for line in &stamp.lines {
            push_text_line(&mut buf, line);
        }
        buf.extend_from_slice(&LEFT);
    }
    buf.resize(buf.len() + feed_lines(&stamp.size_hint), b'\n');
    buf.extend_from_slice(&CUT);
    buf
}
