//! AC9: Each stamp file's `lines` violating the schema (≥13 lines, or
//! line >40 graphemes) is rejected at `add`-time with exit 5;
//! `render`-time of an existing-but-malformed file also exits 5 with the
//! offending line numbered in stderr.
//!
//! Read-only after scaffold.

#[test]
fn ac9_schema_violation_placeholder() {}
