# Eshu API Documentation Stabilization Design Specification

This document details the design of the documentation updates to push the Eshu library to a stable 1.0.0 state.

## 1. `README.md` Example Section

Add a complete, working example to the `### Example` block of `README.md` to demonstrate:
- Flag construction using the builder pattern.
- Value storage types (`StoreType`) and syntaxes (`StoreSyntax`).
- CLI initialization and parsing.
- Querying parsed flags and positional arguments.

### Proposed Code Block in README.md
```rust
use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define flags and options
    let verbose_flag = CliFlag::new("verbose")
        .with_flag_char('v')
        .with_about("Verbose mode", "Print detailed debug information.")
        .build()?;

    let file_flag = CliFlag::new("file")
        .with_flag_char('f')
        .with_about("Target file path", "Specify the path to the input file.")
        .with_store(StoreType::Value, StoreSyntax::Detached)
        .build()?;

    // 2. Configure and parse CLI
    let cli = Cli::new("my-tool")
        .with_version("1.0.0")
        .with_about("A command line interface built with Eshu")
        .add_flag(verbose_flag)
        .add_flag(file_flag)
        .parse(); // Automatically prints error and exits if parsing fails.

    // 3. Extract and use parsed values
    if cli.is_flag_entered("verbose") {
        println!("Verbose logging is enabled.");
    }

    if let Some(store) = cli.get_flag_store("file") {
        if let Some(files) = store.as_value() {
            println!("Processing file: {:?}", files.first());
        }
    }

    // Read any stray positional arguments
    let positionals = cli.get_stray_positional_args();
    if !positionals.is_empty() {
        println!("Additional inputs: {:?}", positionals);
    }

    Ok(())
}
```

---

## 2. Inline Doc Enhancements

### `src/cli/mod.rs`
Improve documentation for `Cli` struct:
```rust
/// Represents the parsed state of the command line interface.
///
/// `Cli` is built using the builder pattern via [`Cli::new`] and [`CliBuilder`].
/// Once parsed (using `.parse()`), it provides methods to query:
/// - Entered flags (`is_flag_entered`, `get_flag_store`)
/// - Positional arguments (`get_stray_positional_args`)
/// - Subcommands (`is_subcommand_entered`, `get_subcommand_cli`)
///
/// It also enables exporting manual pages programmatically using [`Cli::make_manpage`].
```

### `src/control/mod.rs`
Improve documentation for `StoreType` and `StoreSyntax`:
```rust
/// The type of data stored by a parsed flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    /// Stores one or more standalone values (e.g. `--file input.txt`).
    Value,
    /// Stores one or more key-value pairs (e.g. `--define key=value`).
    KeyValue,
}

/// The syntax used to pass value arguments to a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSyntax {
    /// Attached to the flag using an equal sign (e.g., `--flag=value` or `-f=value`).
    Attached,
    /// Detached from the flag, separated by whitespace (e.g., `--flag value` or `-f value`).
    Detached,
}
```

### `src/control/cmd.rs`
Improve documentation for `CliCommand` trait:
```rust
/// Define a custom subcommand with flag parsing and execution capabilities.
///
/// Custom subcommands can be added to the CLI via [`CliBuilder::add_command`].
/// By default, when the parser encounters a subcommand, it builds its corresponding [`Cli`]
/// context and automatically executes the subcommand's `execute` method.
```
