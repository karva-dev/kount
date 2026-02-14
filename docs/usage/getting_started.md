# Getting Started

## Installation

### From source

```bash
cargo install --path crates/kount
```

### From crates.io

```bash
cargo install kount
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
