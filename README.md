# day-stamps

A catalog of special-day stamps for daily-receipt, and the `day-stamp` CLI that looks them up and renders them.

Birthdays, anniversaries, the day a PRD shipped, the day the printer arrived — some days deserve a stamp instead of a haiku or a glyph. `day-stamps` defines the catalog format (date-keyed JSON files), ships the `day-stamp` CLI to manage and render them, and implements the lookup convention that the rest of the daily-receipt family cites. A stamp is a small piece of metadata — title, lines, optional glyph seed — keyed by date, that renders to ESC/POS bytes.

## Why it exists

The day-type classifier needs a clean answer to one question: does *today* have a stamp? That answer has to be stable across years (a birthday recurs) yet allow a specific year to override (the day the printer arrived in 2026 is not every May 22 forever). So the catalog has two file kinds and a precedence rule between them, and `day-stamp which <date>` resolves them the same way every time. Get the lookup right once, in one place, and the renderer never has to reason about dates.

## Install

```sh
cargo install --path . --root ~/.local
```

The binary is `day-stamp` (singular). The catalog lives at `$DAY_STAMP_CATALOG_DIR`, else `$XDG_CONFIG_HOME/daily-receipt/stamps/`, else `~/.config/daily-receipt/stamps/`.

## Quickstart

Seed the starter catalog, then look up and render:

```sh
day-stamp seed                          # write 4 starter stamps (idempotent; never overwrites)
day-stamp which 2026-05-22              # print the stamp id firing on that date, or nothing
day-stamp list --json                  # full stamp metadata as a JSON array
day-stamp render printer-arrives --out stamp.escpos   # ESC/POS bytes for one stamp; <id> may be `today`
day-stamp add --id my-day --title "My Day" --date 2026-07-01 --line "..."
```

`render` output opens with the ESC/POS init `1B 40` and ends with the partial-cut command `1D 56 42 00`, matching the contract every daily-receipt strip honors.

## How it works

Two file kinds, one precedence rule:

| File              | Matches                          | Precedence            |
|-------------------|----------------------------------|-----------------------|
| `MM-DD.json`      | that day every year (recurring)  | base                  |
| `YYYY-MM-DD.json` | that one date                    | shadows the recurring |

So a recurring `05-22.json` fires every May 22, but a `2026-05-22.json` overrides it for 2026 alone. `which` returns the winning stamp id (or nothing, exit 0, on no match). A stamp can be text (`lines`) or glyph-only (`lines` empty, `glyph_seed` set). `add` refuses to overwrite an existing date file (exit 4); a schema violation in `lines` exits 5 and names the offending line on stderr. `seed` writes its starter stamps only where none exist, so it's safe to re-run.

## Where it fits

The special-day branch of the daily-receipt family. [`day-summarize`](https://github.com/j0yen/day-summarize) and [`day-haiku`](https://github.com/j0yen/day-haiku) cite the `which` lookup to decide when a day is `special`; [`daily-receipt`](https://github.com/j0yen/daily-receipt) renders the strip and [`daily-receipt-printer`](https://github.com/j0yen/daily-receipt-printer) prints it; [`daily-receipt-yearend-letter`](https://github.com/j0yen/daily-receipt-yearend-letter) closes the year. Built via the [`autobuilder`](https://github.com/j0yen/autobuilder) pipeline; all nine acceptance criteria have matching tests under `tests/`.

## License

MIT or Apache-2.0, at your option.
