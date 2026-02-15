# Contributing to Kount

Thanks for your interest in contributing to Kount!

## Development Setup

1. Install Rust via [rustup](https://rustup.rs/)

1. Clone the repository:

   ```bash
   git clone https://github.com/MatthewMckee4/kount.git
   cd kount
   ```

1. Build:

   ```bash
   cargo build
   ```

1. Run tests:

   ```bash
   cargo nextest run
   ```

## Project Structure

```text
crates/
├── kount/          # Binary crate (entry point, output formatting)
├── kount_cli/      # CLI argument parsing (clap)
└── kount_count/    # Core counting logic and directory walking
```

## Running Checks

Before submitting a PR, make sure all checks pass:

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo nextest run
```

If you have [pre-commit](https://pre-commit.com/) installed:

```bash
pre-commit install
pre-commit run --all-files
```

## Guidelines

- Follow existing code style and patterns
- Add integration tests for new features in `crates/kount/tests/it/`
- Keep commits focused and write clear commit messages
- Avoid `.unwrap()` in library code — use proper error handling
- Run clippy and fix all warnings before submitting
