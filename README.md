<div align="center">

# Flit

**Read faster. One word at a time.**

*A modern RSVP terminal reader — a complete rewrite of [pasky/speedread](https://github.com/pasky/speedread) in Rust.*

[![CI](https://github.com/kuanysh228/Flit/actions/workflows/ci.yml/badge.svg)](https://github.com/kuanysh228/Flit/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

</div>

---

Flit uses **Rapid Serial Visual Presentation (RSVP)** — words flash one at a time, each aligned so your eyes never move. No scrolling. No scanning. Just reading.

```
                              v
                       read ·every· word
                       
──────────────────────────────────────────────────
 300 wpm  ·  42%  ·  03:12 elapsed  ·  ▶ playing
```

The red letter marks the **Optimal Recognition Point** — the spot your brain locks onto fastest.

---

## Install

```bash
cargo install flit
```

Or grab a pre-built binary from [Releases](https://github.com/kuanysh228/Flit/releases).

**Optional format support:**

```bash
cargo install flit --features full   # adds pdf, epub, docx, html, fb2, rtf
```

## Quickstart

```bash
flit read book.txt          # start reading, auto-resume on next launch
flit read book.txt -w 400   # set speed (words per minute)
flit read --stdin           # pipe mode: cat article.txt | flit read --stdin
flit list                   # all files you've opened, with progress
flit stats week             # reading stats for the last 7 days
```

## Controls

| Key | Action |
|-----|--------|
| `Space` | Pause / resume |
| `j` / `[` | Slow down (×0.9) |
| `k` / `]` | Speed up (×1.1) |
| `h` | Back 1 word |
| `l` | Forward 1 word |
| `b` | Back 1 sentence |
| `w` | Forward 1 sentence |
| `{` / `}` | Back / forward paragraph |
| `gg` | Go to start |
| `G` | Go to end |
| `/` | Search |
| `n` / `N` | Next / previous match |
| `:` | Command mode |
| `?` | Help overlay |
| `q` | Quit (saves position) |

**Command mode** (press `:`):

```
:set wpm 500        change speed
:goto 50%           jump to midpoint
:goto 1234          jump to word 1234
:stats              session stats overlay
:q / :wq / :q!      quit
```

## Features

- **Smart bookmarks** — resumes exactly where you left off, survives file edits and renames
- **Vim keybindings** — modal input, count prefixes (`5l` = forward 5 words), search
- **Format support** — `.txt`, `.md` built-in; `.pdf`, `.epub`, `.docx`, `.html`, `.fb2`, `.rtf` via `--features full`
- **Reading stats** — words read, average WPM, peak WPM, streaks — all local, no telemetry
- **Pipe mode** — drop-in replacement for the original `speedread`: `cat file | flit read --stdin`
- **Single binary** — no runtime dependencies, ~4 MB, works on Linux / macOS / Windows

## Configuration

Flit reads `~/.config/flit/config.toml` (XDG-compliant):

```toml
[reading]
default_wpm = 300
min_wpm     = 100
max_wpm     = 1200

[ui]
theme = "dark"   # "dark" or "light"

[storage]
# db_path = "/custom/path/db.sqlite3"
```

All keys are optional — defaults apply if omitted.

## vs. original speedread

| | speedread | Flit |
|---|---|---|
| Language | Perl | Rust |
| Distribution | requires Perl runtime | single static binary |
| Formats | plain text only | txt, md, pdf, epub, docx, html, fb2, rtf |
| Bookmarks | `-r N` (breaks on edits) | sqlite + content fingerprint |
| Controls | `[`, `]`, space | full vim mode + `:` commands + search |
| Stats | none | per-session and historical |
| Config | hardcoded | `~/.config/flit/config.toml` |
| Tests | none | unit, property-based, integration |
| Maintained | abandoned 2016 | active |

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

Based on the original work of [Petr Baudis](https://github.com/pasky/speedread).
