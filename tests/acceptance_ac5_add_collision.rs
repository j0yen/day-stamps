//! AC5: `day-stamp add --id foo --title T --date 2026-05-30 --line "Hello"`
//! creates `2026-05-30.json` with the expected shape; running again with
//! the same `--id` and date errors out with exit 4 (do not silently
//! overwrite).
//!
//! Read-only after scaffold.

#[test]
fn ac5_add_collision_placeholder() {}
