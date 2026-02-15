# Getting Started

## Installation

```bash
# On macOS and Linux.
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/karva-dev/kount/releases/latest/download/kount-installer.sh | sh

# On Windows.
powershell -ExecutionPolicy Bypass -c "irm https://github.com/karva-dev/kount/releases/latest/download/kount-installer.ps1 | iex"
```

## Quick Start

Count lines in the current directory:

```bash
kount .
```

Count only Rust files:

```bash
kount --ext rs .
```

Count lines in specific files:

```bash
kount src/main.rs src/lib.rs
```

Get a summary by extension:

```bash
kount --summary .
```

Output as JSON for scripting:

```bash
kount --json . | jq '.total_lines'
```

Sort files by name instead of line count:

```bash
kount --sort name .
```

Include hidden files and ignore `.gitignore`:

```bash
kount --no-ignore .
```
