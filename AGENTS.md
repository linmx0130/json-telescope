# json-telescope — Agent Guide

## Project Overview

`json-telescope` is a minimal Rust TUI for interactively browsing JSONL files. It renders a scrollable list of entries; pressing Enter on an entry opens an expandable tree view of its JSON structure.

## Build & Run

```bash
# Build
cargo build --release

# Run with a JSONL file
cargo run -- data/sample.jsonl
```

The binary takes exactly one positional argument: the path to a `.jsonl` file. Empty lines are skipped. Parse errors are fatal and printed to stderr.

## Architecture

| File | Responsibility |
|------|----------------|
| `src/main.rs` | CLI argument parsing, file I/O, terminal init/shutdown, event loop |
| `src/app.rs` | App state machine (`Screen::List` vs `Screen::Inspect`), key-event dispatch |
| `src/json_tree.rs` | Tree data model (`TreeState`, `VisibleNode`, `NodeKind`), flattening, expand/collapse logic |
| `src/ui.rs` | All ratatui drawing code (`draw_list`, `draw_inspect`, `render_node`) |

### Key Types

- **`App`** — holds `Vec<serde_json::Value>` entries, selected index, and current screen.
- **`Screen::Inspect(TreeState)`** — tree view state for the currently selected entry.
- **`TreeState`** — tracks the JSON root, which paths are expanded (`HashSet<Vec<PathSegment>>`), selected row, and scroll offset.
- **`VisibleNode`** — a flattened representation of a JSON node at a given depth, produced by `TreeState::flatten()`.

## Dependencies

- `ratatui = "0.29"` — TUI widgets and layout
- `crossterm = "0.28"` — raw mode, alternate screen, keyboard/mouse events
- `serde_json = "1"` — JSON parsing

## Key Behaviors & Constraints

- **Edition:** Rust 2024.
- **Input:** JSONL (one JSON object per line).
- **Fatal errors:** Missing argument, file read failure, JSON parse failure, empty file after stripping blank lines.
- **Vim-style bindings:** `j`/`k` up/down, `l`/Enter expand/open, `h`/Left collapse, `q`/Esc quit/back.
- **Mouse capture is enabled** but no mouse handlers are wired up.
- **Scroll offset:** `TreeState::ensure_visible` and `clamp_selected` are called inside `draw_inspect` before rendering.

## Conventions

- Keep modules single-responsibility: state in `app.rs` + `json_tree.rs`, rendering in `ui.rs`, glue in `main.rs`.
- Use `crossterm` events directly; do not abstract into a custom event type unless the app grows significantly.
- Color scheme is hard-coded in `ui.rs` (cyan keys, yellow indices, green strings, yellow numbers, magenta booleans, dark-gray null).
- Path segments use `PathSegment::Key(String)` and `PathSegment::Index(usize)`.

## Testing Notes

There are currently no automated tests. To verify changes manually:

```bash
cargo run -- data/sample.jsonl
```
