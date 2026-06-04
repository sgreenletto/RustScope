# RustScope

RustScope is a local Rust project code structure analysis and quality report tool. Given a Rust project path, it recursively scans `.rs` files, calculates line metrics, recognizes basic Rust items, estimates simple function complexity, analyzes `use` dependencies, and generates terminal, Markdown, HTML, or lightweight TUI views.

## Features

- `analyze` command with `terminal`, `markdown`, and `html` report formats.
- `tui` command for an interactive terminal dashboard.
- Recursive `.rs` scanning with `target/` and `.git/` ignored.
- Multi-threaded file analysis with `std::thread` and `mpsc`.
- Total, code, comment, and blank line metrics.
- Basic item recognition for `fn`, `struct`, `enum`, `trait`, `mod`, and `impl`.
- Heuristic function complexity scoring for `if`, `match`, `for`, `while`, `loop`, and `?`.
- Simple `use` dependency analysis for paths such as `crate::`, `super::`, `std::`, and local modules.
- Markdown and HTML reports with Mermaid dependency graph source.
- `--output` support with automatic parent directory creation.
- Unified `RustScopeError` error handling.

## Project Structure

```text
src/
  main.rs        Program entry point
  cli.rs         Command line parsing
  analyzer.rs    Project analysis orchestration and parallel workers
  scanner.rs     Recursive Rust file scanner
  metrics.rs     Line metrics and function complexity
  parser.rs      Basic Rust item recognition
  dependency.rs  use dependency analysis
  report.rs      Terminal, Markdown, HTML, and Mermaid report generation
  output.rs      Report file writing
  tui.rs         Lightweight terminal dashboard
  model.rs       Core data structures
  error.rs       Unified error types
```

## Build

```bash
cargo build
```

## Run

Terminal report:

```bash
cargo run -- analyze ./examples/demo_project
```

Explicit terminal format:

```bash
cargo run -- analyze ./examples/demo_project --format terminal
```

Markdown report:

```bash
cargo run -- analyze ./examples/demo_project --format markdown --output reports/report.md
```

HTML report:

```bash
cargo run -- analyze ./examples/demo_project --format html --output reports/report.html
```

TUI dashboard:

```bash
cargo run -- tui ./examples/demo_project
```

Press `q` and Enter to exit the lightweight TUI dashboard.

Short analyze form:

```bash
cargo run -- ./examples/demo_project
```

## Example Terminal Output

```text
RustScope Analysis Report
=========================

Project: ./examples/demo_project

Files analyzed: 3

Parallel analysis: enabled
Worker threads: 3

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
- `markdown`: Markdown report with summary tables and Mermaid graph source.
- `html`: static HTML report with CSS, tables, escaped text, and Mermaid graph source.

HTML reports do not start a web service. Open `reports/report.html` directly in a browser. The current HTML report displays Mermaid as source code; it can be copied into a Mermaid previewer if rendered graph output is needed.

## Dependency Analysis

RustScope performs lightweight `use` dependency analysis after comment stripping. It recognizes direct `use` statements such as:

```rust
use crate::scanner;
use crate::parser::parse_file;
use super::model::ProjectAnalysis;
use std::collections::HashMap;
```

The Markdown and HTML reports include both a dependency table and Mermaid graph source. Node IDs are generated as safe values such as `node0`, while labels preserve the original module or path names.

## Demo Reports

Generated files under `reports/` can be used as presentation material:

```bash
cargo run -- analyze ./examples/demo_project --format markdown --output reports/report.md
cargo run -- analyze ./examples/demo_project --format html --output reports/report.html
```

If `reports/` does not exist, RustScope creates it automatically.

## Test

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

On Windows, if the default `target/` directory has an incremental compilation permission issue, use:

```powershell
cargo test --target-dir "$env:TEMP\rustscope-target"
cargo clippy --target-dir "$env:TEMP\rustscope-target" -- -D warnings
```

## Current Limits

- RustScope uses lightweight text rules, not a complete Rust compiler AST.
- Function complexity is heuristic and intentionally simple.
- Dependency analysis reads direct `use` statements and does not fully expand grouped imports.
- The TUI is a single-page dashboard, not a full multi-page terminal app.
- Mermaid output is source text, not exported images.

## Future Ideas

- More precise AST-based analysis.
- Incremental cache.
- More advanced TUI interactions.
- Dependency graph image export.
