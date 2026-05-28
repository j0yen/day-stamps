//! AC7: `day-stamp render` with no `lines` and `glyph_seed: 42` renders
//! a 24×24 raster glyph (reuses daily-receipt's renderer via dep, or
//! shells out to `daily-receipt render` with a fake summary — either is
//! acceptable; pick the simpler path).
//!
//! Read-only after scaffold.

#[test]
fn ac7_glyph_only_placeholder() {}
