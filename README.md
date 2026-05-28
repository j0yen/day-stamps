# day-stamps

Special-day stamp catalog + lookup for [daily-receipt](https://github.com/j0yen/daily-receipt).

Birthdays, anniversaries, the day a PRD shipped, the day the printer
arrived — special days deserve a stamp instead of a haiku or a glyph.
This crate ships the catalog format (date-specific + recurring JSON
files), the `day-stamp` CLI (`add | list | render | which | seed`), and
the lookup convention that `day-summarize` and `day-haiku` cite.

A stamp is a tiny piece of pre-rendered ESC/POS bytes (or a glyph spec)
plus metadata, keyed by date.

## Status

Autobuilder scaffold (iter-0). The implementation lands in subsequent
iterations under `autobuilder/daily-receipt-stamps`. PRD:
[`PRD-daily-receipt-stamps.md`](https://github.com/j0yen/autobuilder).

## Acceptance criteria (MUST)

1. `day-stamp seed` writes 4 starter stamps to an empty catalog;
   re-running does not overwrite.
2. `day-stamp which <date>` prints the matching stamp id, or nothing
   on no-match (exit 0).
3. Recurring `MM-DD.json` matches across years.
4. Date-specific `YYYY-MM-DD.json` shadows recurring for the same MM-DD.
5. `day-stamp add` errors with exit 4 on id+date collision.
6. `day-stamp render <id> --out <path>` emits valid ESC/POS bytes
   (begins with `0x1B 0x40`, ends with `0x1D 0x56 0x42 0x00`).
7. Glyph-only render works when `lines` is empty and `glyph_seed` is set.
8. `day-stamp list --json` is a sortable JSON array.
9. Schema violations exit 5 with stderr naming the offending line.

## Install

After implementation lands:

```
cargo install --path . --root ~/.local
```

The binary is `day-stamp` (singular). Catalog default:
`$XDG_CONFIG_HOME/daily-receipt/stamps/`, fallback
`~/.claude/daily-receipt/stamps/`.

## License

Dual MIT / Apache-2.0.
