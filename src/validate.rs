//! Stamp `lines` schema validation — pure, dependency-light so it can be
//! `#[path]`-included by the proptest harness.
//!
//! Rules (PRD §schema, AC9): at most 12 lines; each line at most 40
//! graphemes. The lower bound (≥1 line) is intentionally not enforced so
//! glyph-only stamps (`glyph_seed` set, no `lines`) remain valid per AC7.

use unicode_segmentation::UnicodeSegmentation;

/// Maximum number of `lines` a stamp may carry.
pub const MAX_LINES: usize = 12;
/// Maximum grapheme width of a single stamp line.
pub const MAX_GRAPHEMES: usize = 40;

/// A schema violation, carrying the 1-indexed offending line and a reason.
#[derive(Debug, Clone)]
pub struct LineError {
    /// 1-indexed line number the violation refers to.
    pub line: usize,
    /// Human-readable reason, e.g. `>40 graphemes (got 41)`.
    pub reason: String,
}

/// Validate a stamp's `lines` against the schema bounds.
///
/// # Errors
/// Returns [`LineError`] for the first line exceeding [`MAX_GRAPHEMES`], or a
/// whole-list error when the count exceeds [`MAX_LINES`].
pub fn validate_lines(lines: &[String]) -> Result<(), LineError> {
    if lines.len() > MAX_LINES {
        return Err(LineError {
            line: MAX_LINES + 1,
            reason: format!("too many lines (got {}, max {MAX_LINES})", lines.len()),
        });
    }
    for (idx, line) in lines.iter().enumerate() {
        let count = line.graphemes(true).count();
        if count > MAX_GRAPHEMES {
            return Err(LineError {
                line: idx + 1,
                reason: format!(">{MAX_GRAPHEMES} graphemes (got {count})"),
            });
        }
    }
    Ok(())
}
