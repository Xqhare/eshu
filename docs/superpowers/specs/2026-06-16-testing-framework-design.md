# Design Spec: Rework and Expand Testing Framework

## Goals
- Clean up `tests/parser_tests.rs` by partitioning it into separate, focused submodules under a new `tests/parser_tests/` directory.
- Maintain fast compile times by preserving `parser_tests.rs` as a single cargo integration test binary.
- Expand test coverage for core functionality and edge cases, specifically key-value parsing, `XffValue` casting, nested subcommands, and configuration validation.

## Test Directory Structure

```
tests/
├── builder_tests.rs
├── subcmd_tests.rs
├── parser_tests.rs (declares submodules below)
└── parser_tests/
    ├── basic.rs
    ├── end_of_flag.rs
    ├── errors.rs
    ├── key_value.rs (new)
    ├── regression.rs
    ├── store.rs
    └── xff.rs (new)
```

## Coverage Plan

### 1. Parser Splitting & Cleanup
Migrate existing tests from `tests/parser_tests.rs` to modular files:
- `basic.rs`: Basic long flag parsing and partial/ambiguous flag checks.
- `end_of_flag.rs`: Processing of the end-of-flag marker (`--`).
- `store.rs`: Attached and detached store syntax behavior.
- `regression.rs`: Verification of past bug regression scenarios.
- `errors.rs`: Verification of parser errors (like missing arguments).

### 2. New Test Additions

#### `tests/parser_tests/key_value.rs`
- **Detached Key-Value**: Test parsing of key-value pairs separated by whitespace (e.g. `--config key=value`).
- **Attached Key-Value**: Test parsing of key-value pairs separated by an equals sign (e.g. `--config=key=value` and `-c=key=value`).
- **Multiple Key-Values**: Validate that multiple occurrences of the same key-value flag correctly aggregate into a single map.
- **Malformed Input**: Verify that passing a non-key-value argument to a key-value flag returns a proper syntax error.

#### `tests/parser_tests/xff.rs`
- **Conversion to XffValue**: Test conversions for all three variants of `Store` into `XffValue`:
  - `Store::Exists` -> `XffValue::Boolean(true)`
  - `Store::Value(Vec<String>)` -> `XffValue::Array`
  - `Store::KeyValue(BTreeMap<String, String>)` -> `XffValue::Object`

#### `tests/subcmd_tests.rs`
- **Nested Subcommands**: Verify commands configured like `app parent child --flag` are parsed correctly, and command execution and state are isolated.
- **Invalid Commands**: Test parser behavior when receiving unrecognized commands.

#### `tests/builder_tests.rs`
- **Duplicate Flag Names**: Ensure adding two flags with identical names returns a builder error.
- **Duplicate Short Flags**: Ensure adding two flags with identical short flag characters returns a builder error.
- **Flag Name Whitespace / Dashes**: Validate that flags with whitespace or invalid starting characters are correctly rejected during build.
