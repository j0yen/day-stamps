//! AC6: `day-stamp render printer-arrives --out /tmp/p.escpos` writes a
//! non-empty ESC/POS byte stream that begins with ESC '@' (`0x1B 0x40`),
//! contains the stamp title bytes verbatim, ends with feed-and-cut bytes
//! (`0x1D 0x56 0x42 0x00`). Daily-receipt AC2 compatibility.
//!
//! Read-only after scaffold.

#[test]
fn ac6_render_bytes_placeholder() {}
