# fyx-tui

Minimal, ranger-inspired terminal file manager built with Rust, ratatui, and crossterm.

## Current iteration
- 3-pane layout: parent / current / preview
- Vim-style navigation (`j`, `k`, `h`, `l`, `gg`, `G`, `:` `/`)
- Mouse navigation (scroll, click select, double-click enter)
- Async preview worker (crossbeam channel + thread)

## Run

```bash
cargo run
```

## Quit
- `q`
