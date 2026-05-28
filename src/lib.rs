//! day-stamps — special-day stamp catalog, lookup, and ESC/POS rendering.
//!
//! A *stamp* is a tiny piece of pre-rendered receipt content (centered text
//! lines and/or a raster glyph) keyed by date. Date-specific stamps
//! (`<YYYY-MM-DD>.json`) fire once; recurring stamps (`<MM-DD>.json`) fire
//! every year. The date-specific file shadows the recurring one.
//!
//! Upstream consumers (`day-summarize`, `day-haiku`) call [`which`] to learn
//! whether today is a special day; the printer wrapper calls
//! [`render::render`] to obtain the byte stream.

#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod catalog;
pub mod render;
pub mod validate;

use chrono::NaiveDate;

use crate::catalog::{Res, Stamp};

/// Look up the stamp (if any) firing on `date`, resolving the catalog
/// directory from the environment (`DAY_STAMP_CATALOG_DIR`, then
/// `XDG_CONFIG_HOME`, then `~/.config`).
///
/// This is the public entry point cited by the upstream lookup contract:
/// `day_stamps::which(today)`.
///
/// # Errors
/// Returns an error when the catalog directory cannot be resolved.
pub fn which(date: NaiveDate) -> Res<Option<Stamp>> {
    let dir = catalog::catalog_dir()?;
    Ok(catalog::which(&dir, date))
}
