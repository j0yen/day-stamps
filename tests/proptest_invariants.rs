//! Property-based invariant tests.
//!
//! Read-only after scaffold. The edit-agent must NOT modify proptests.
//!
//! Domain invariant: `validate::validate_lines` accepts a `lines` vector if
//! and only if it carries at most `MAX_LINES` entries AND every entry is at
//! most `MAX_GRAPHEMES` graphemes wide. When it rejects, the reported line
//! number is 1-indexed and within `1..=len+1`.

use day_stamps::validate::{self, MAX_GRAPHEMES, MAX_LINES};
use proptest::prelude::*;

proptest! {
    /// Acceptance is exactly equivalent to the schema predicate.
    #[test]
    fn accept_iff_within_bounds(lines in proptest::collection::vec("[a-z ]{0,60}", 0..20)) {
        let within = lines.len() <= MAX_LINES
            && lines.iter().all(|l| l.chars().count() <= MAX_GRAPHEMES);
        // ASCII chars are one grapheme each, so char-count == grapheme-count here.
        prop_assert_eq!(validate::validate_lines(&lines).is_ok(), within);
    }

    /// A reported error always points at a 1-indexed line within range.
    #[test]
    fn error_line_in_range(lines in proptest::collection::vec("[a-z]{0,60}", 0..20)) {
        if let Err(e) = validate::validate_lines(&lines) {
            prop_assert!(e.line >= 1);
            prop_assert!(e.line <= lines.len() + 1);
            prop_assert!(!e.reason.is_empty());
        }
    }
}
