# RustScope Analysis Report

## Project Summary

- Project Path: ./examples/demo_project
- Files Analyzed: 3

## Line Metrics

| Metric | Count |
|---|---:|
| Total Lines | 98 |
| Code Lines | 85 |
| Comment Lines | 0 |
| Blank Lines | 13 |

## Code Items

| Item | Count |
|---|---:|
| Functions | 8 |
| Structs | 2 |
| Enums | 1 |
| Traits | 1 |
| Modules | 2 |
| Impl Blocks | 3 |

## Top Complex Functions

| Rank | Function | File | Line | Complexity |
|---:|---|---|---:|---:|
| 1 | analyze | ./examples/demo_project\src\analyzer.rs | 12 | 5 |
| 2 | run_demo | ./examples/demo_project\src\main.rs | 16 | 4 |
| 3 | run | ./examples/demo_project\src\analyzer.rs | 39 | 2 |
| 4 | new | ./examples/demo_project\src\analyzer.rs | 8 | 1 |
| 5 | run | ./examples/demo_project\src\domain.rs | 13 | 1 |

## Module Dependencies

| From | To |
|---|---|
| ./examples/demo_project/src/analyzer.rs | crate::domain::{DemoItem, DemoState, Runnable} |
| ./examples/demo_project/src/main.rs | analyzer::Analyzer |
| ./examples/demo_project/src/main.rs | domain::{DemoItem, DemoState, Runnable} |
