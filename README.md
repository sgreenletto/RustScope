# RustScope

RustScope is a local Rust project structure analysis and quality report tool. Given a Rust project path, it recursively scans `.rs` files, summarizes line metrics, recognizes basic Rust code items, estimates simple function complexity, and prints a terminal report.

## Current Features

- Parse `analyze <project-path>` command line input.
- Recursively scan Rust source files.
- Ignore `target/` and `.git/` directories.
- Count total, code, comment, and blank lines.
- Recognize `fn`, `struct`, `enum`, `trait`, `mod`, and `impl`.
- Estimate simple function complexity from `if`, `match`, `for`, `while`, `loop`, and `?`.
- Print the top 5 most complex functions.

## Project Structure

```text
src/
  main.rs      Program entry point
  cli.rs       Command line parsing
  scanner.rs   Recursive Rust file scanner
  metrics.rs   Line metrics and function complexity
  parser.rs    Basic Rust item recognition
  report.rs    Terminal report generation
  model.rs     Core data structures
  error.rs     Basic error types
```

## Build

```bash
cargo build
```

## Run

Preferred command:

```bash
cargo run -- analyze ./examples/demo_project
```

Short form:

```bash
cargo run -- ./examples/demo_project
```

## Example Output

```text
RustScope Analysis Report
=========================

Project: ./examples/demo_project

Files analyzed: 3

Line Metrics:
- Total lines: 98
- Code lines: 85
- Comment lines: 0
- Blank lines: 13

Code Items:
- Functions: 8
- Structs: 2
- Enums: 1
- Traits: 1
- Modules: 2
- Impl blocks: 3

Top Complex Functions:
1. analyze              ./examples/demo_project/src/analyzer.rs:12    complexity: 5
2. run_demo             ./examples/demo_project/src/main.rs:16        complexity: 4
```

## Test

```bash
cargo test
```

Recommended checks:

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## Future Plans

- Markdown reports
- HTML reports
- Multi-threaded scanning
- TUI interface
