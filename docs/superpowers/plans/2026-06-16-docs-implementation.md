# Eshu API Documentation Stabilization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a comprehensive example to `README.md` and enhance public inline docs across `Cli`, `StoreType`, `StoreSyntax`, and `CliCommand`.

**Architecture:** Update `README.md` and Rust source docstrings to stabilize the API docs and fix compile-time doc warnings.

**Tech Stack:** Rust, `eshu` library docs parser.

---

### Task 1: Add Example to README.md

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Write code block in README.md**
  Locate the empty `### Example` block in `README.md` (around line 131) and replace it with:
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
          .parse();

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

- [ ] **Step 2: Run doctests to verify it compiles and runs**
  Run: `cargo test --doc`
  Expected: PASS

- [ ] **Step 3: Commit changes**
  ```bash
  git add README.md
  git commit -m "docs: add comprehensive usage example to README"
  ```

---

### Task 2: Enhance src/cli/mod.rs Inline Docs

**Files:**
- Modify: `src/cli/mod.rs:13-42`

- [ ] **Step 1: Update the docstring of the Cli struct**
  Locate lines 13-19 of `src/cli/mod.rs`:
  ```rust
  /// Generate a command line interface
  #[expect(
  ```
  Replace the docstring of `Cli` to explain how it holds state:
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
  #[expect(
  ```

- [ ] **Step 2: Build documentation to verify syntax**
  Run: `cargo doc --no-deps`
  Expected: PASS with no syntax or missing reference warnings

- [ ] **Step 3: Commit changes**
  ```bash
  git add src/cli/mod.rs
  git commit -m "docs(cli): enhance Cli struct documentation"
  ```

---

### Task 3: Enhance src/control/mod.rs Inline Docs

**Files:**
- Modify: `src/control/mod.rs:9-40`

- [ ] **Step 1: Update docstrings for StoreType and StoreSyntax**
  Locate lines 9-40 of `src/control/mod.rs`:
  ```rust
  /// The type of the store
  ...
  /// The syntax of the store
  ...
  ```
  Replace with:
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
  ///
  /// # Usage
  ///
  /// If a flag is attached, the calling syntax will be `--flag-name=value`.
  /// If the flag has a required store, the calling syntax will be `--flag-name=value`, otherwise the calling syntax will be `--flag-name[=value]`.
  ///
  /// If a flag is detached, the calling syntax will be `--flag-name value`.
  /// If the flag has a required store, the calling syntax will be `--flag-name value`, otherwise the calling syntax will be `--flag-name [value]`.
  /// Please note, this also allows `--flag-name "a value with spaces"`, which is not possible with an attached flag (blame POSIX).
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum StoreSyntax {
      /// Attached to the flag; `--flag-name=value` or `--flag-name=key=value`
      Attached,
      /// Detached from the flag; `--flag-name value` or `--flag-name "a value with spaces"` or `--flag-name key=value`
      Detached,
  }
  ```

- [ ] **Step 2: Build documentation to verify syntax**
  Run: `cargo doc --no-deps`
  Expected: PASS

- [ ] **Step 3: Commit changes**
  ```bash
  git add src/control/mod.rs
  git commit -m "docs(control): enhance StoreType and StoreSyntax documentation"
  ```

---

### Task 4: Enhance src/control/cmd.rs Inline Docs

**Files:**
- Modify: `src/control/cmd.rs:22-44`

- [ ] **Step 1: Update the docstring of the CliCommand trait**
  Locate lines 22-23 of `src/control/cmd.rs`:
  ```rust
  /// Define a custom command
  pub trait CliCommand<'c> {
  ```
  Replace with:
  ```rust
  /// Define a custom subcommand with flag parsing and execution capabilities.
  ///
  /// Custom subcommands can be added to the CLI via [`CliBuilder::add_command`].
  /// By default, when the parser encounters a subcommand, it builds its corresponding [`Cli`]
  /// context and automatically executes the subcommand's `execute` method.
  pub trait CliCommand<'c> {
  ```

- [ ] **Step 2: Run all doctests and generate final documentation**
  Run: `cargo test --doc && cargo doc --no-deps`
  Expected: PASS with no warnings or errors

- [ ] **Step 3: Commit changes**
  ```bash
  git add src/control/cmd.rs
  git commit -m "docs(control): enhance CliCommand trait documentation"
  ```
