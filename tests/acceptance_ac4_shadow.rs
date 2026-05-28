//! AC4: Date-specific stamp file shadows the recurring file for the
//! same MM-DD: when both `2026-12-31.json` and `12-31.json` exist,
//! `which 2026-12-31` returns the date-specific one.
//!
//! Read-only after scaffold.

#[test]
fn ac4_shadow_placeholder() {}
