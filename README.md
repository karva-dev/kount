# kount

A fast line counter for files and directories, written in Rust.

## Installation

```bash
cargo install kount
```

## Usage

```bash
# Count lines in the current directory
kount .

# Count only Rust and Python files
kount --ext rs,py .

# Sort by filename
kount --sort name src/

# JSON output for scripting
kount --json . | jq '.total_lines'

# Summary with per-extension breakdown
kount --summary .

# Include hidden files, ignore .gitignore
kount --no-ignore .
```

## Output

```
 Lines  File
──────  ────────────────────────────
   152  src/walker.rs
    87  src/counter.rs
    42  src/types.rs
──────  ────────────────────────────
   281  total (3 files)
```

## Options

| Option | Description |
|--------|-------------|
| `-e, --ext <EXT>` | Filter by extension (comma-separated) |
| `-g, --glob <GLOB>` | Filter by glob pattern (repeatable) |
| `-s, --sort <SORT>` | Sort: `lines` (default), `name`, `none` |
| `--no-ignore` | Include hidden files, ignore `.gitignore` |
| `--json` | Output as JSON |
| `--summary` | Show totals and per-extension breakdown |

## License

MIT
