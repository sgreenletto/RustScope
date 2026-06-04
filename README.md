# RustScope

RustScope is a local Rust project structure analysis and quality report tool. Given a Rust project path, it recursively scans `.rs` files, summarizes line metrics, recognizes basic Rust code items, estimates simple function complexity, analyzes `use` dependencies, and generates terminal or Markdown reports.

## Current Features

- Parse `analyze <project-path>` command line input.
- Recursively scan Rust source files.
- Ignore `target/` and `.git/` directories.
- Count total, code, comment, and blank lines.
- Recognize `fn`, `struct`, `enum`, `trait`, `mod`, and `impl`.
- Estimate simple function complexity from `if`, `match`, `for`, `while`, `loop`, and `?`.
- Analyze simple module dependencies from `use` statements such as `crate::`, `super::`, and `std::`.
- Print the top 5 most complex functions.
- Generate reports in `terminal` or `markdown` format.
- Write reports to an output file with `--output`.

## Project Structure

```text
src/
  main.rs      Program entry point
  cli.rs       Command line parsing
  scanner.rs   Recursive Rust file scanner
  metrics.rs   Line metrics and function complexity
  parser.rs    Basic Rust item recognition
  dependency.rs Basic use dependency analysis
  report.rs    Terminal and Markdown report generation
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

Terminal report:

```bash
cargo run -- analyze ./examples/demo_project --format terminal
```

Markdown report printed to the terminal:

```bash
cargo run -- analyze ./examples/demo_project --format markdown
```

Markdown report written to a file:

```bash
cargo run -- analyze ./examples/demo_project --format markdown --output report.md
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

Module Dependencies:
./examples/demo_project/src/analyzer.rs -> crate::domain::{DemoItem, DemoState, Runnable}
./examples/demo_project/src/main.rs -> analyzer::Analyzer
./examples/demo_project/src/main.rs -> domain::{DemoItem, DemoState, Runnable}
```

## Report Formats

- `terminal`: default plain terminal report.
- `markdown`: Markdown report with project summary, line metrics, code item counts, top complex functions, and module dependencies.

Unsupported formats such as `html` return a clear error. HTML generation is planned for a later stage.

## Dependency Analysis

RustScope performs simple `use` dependency analysis. It recognizes direct `use` statements after comments are stripped, including paths that start with `crate::`, `super::`, `std::`, or local module names. This stage reports dependency rows only; it does not generate dependency graphs.

## Test

```bash
cargo test
```

Recommended checks:

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## Future Plans

- Multi-threaded analysis
- HTML reports
- Mermaid dependency graph
- TUI interface
