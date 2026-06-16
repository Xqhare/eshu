# Eshu CLI Examples Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create three functional examples in the `examples/` directory (`basic.rs`, `git_subcommands.rs`, and `manpage.rs`) demonstrating eshu's API capabilities.

**Architecture:** Three standalone Rust binary files placed in the `examples/` folder. They will import the `eshu` crate and implement standard executable patterns.

**Tech Stack:** Rust 2024 edition, `eshu` library, `std` library.

---

### Task 1: Basic CLI Example

**Files:**
- Create: `examples/basic.rs`

- [ ] **Step 1: Write the minimal template and check compilation**
  Create `examples/basic.rs` with a simple main function:
  ```rust
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      println!("Basic template");
      Ok(())
  }
  ```

- [ ] **Step 2: Compile the example**
  Run: `cargo check --example basic`
  Expected: PASS

- [ ] **Step 3: Write the full basic CLI implementation**
  Replace the contents of `examples/basic.rs` with the full parser demonstration:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

  fn main() -> Result<(), Box<dyn std::error::Error>> {
      // 1. Build individual flags
      let verbose_flag = CliFlag::new("verbose")
          .with_flag_char('v')
          .with_about("Enable verbose output", "Print detailed execution logs.")
          .build()?;

      let output_flag = CliFlag::new("output")
          .with_flag_char('o')
          .with_about("Output file path", "Specify the path where results will be saved.")
          .with_store(StoreType::Value, StoreSyntax::Detached)
          .build()?;

      let define_flag = CliFlag::new("define")
          .with_flag_char('D')
          .with_about("Define variables", "Define config options using key=value pairs.")
          .with_store(StoreType::KeyValue, StoreSyntax::Detached)
          .build()?;

      // 2. Build the main CLI interface
      let cli = Cli::new("copy-cat")
          .with_version(env!("CARGO_PKG_VERSION"))
          .with_about("A simple file processor utility using eshu")
          .add_flag(verbose_flag)
          .add_flag(output_flag)
          .add_flag(define_flag)
          .parse();

      // 3. Retrieve parsed options
      if cli.is_flag_entered("verbose") {
          println!("Verbose mode is enabled!");
      }

      if let Some(store) = cli.get_flag_store("output") {
          if let Some(values) = store.as_value() {
              println!("Output targets: {:?}", values);
          }
      }

      if let Some(store) = cli.get_flag_store("define") {
          if let Some(kv) = store.as_key_value() {
              for (key, val) in kv {
                  println!("Defined: {} = {}", key, val);
              }
          }
      }

      // Retrieve positional arguments
      let positionals = cli.get_stray_positional_args();
      if !positionals.is_empty() {
          println!("Positional arguments: {:?}", positionals);
      } else {
          println!("No positional arguments provided.");
      }

      Ok(())
  }
  ```

- [ ] **Step 4: Verify the basic example execution**
  Run: `cargo run --example basic -- -v -o output.txt -D host=localhost -D port=8080 input1.txt input2.txt`
  Expected:
  ```
  Verbose mode is enabled!
  Output targets: ["output.txt"]
  Defined: host = localhost
  Defined: port = 8080
  Positional arguments: ["input1.txt", "input2.txt"]
  ```

- [ ] **Step 5: Commit changes**
  ```bash
  git add examples/basic.rs
  git commit -m "add(examples): implement basic cli example"
  ```

---

### Task 2: Subcommands CLI Example

**Files:**
- Create: `examples/git_subcommands.rs`

- [ ] **Step 1: Write template for git_subcommands.rs**
  Create `examples/git_subcommands.rs` with the subcommand template:
  ```rust
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      println!("Subcommand template");
      Ok(())
  }
  ```

- [ ] **Step 2: Compile the example**
  Run: `cargo check --example git_subcommands`
  Expected: PASS

- [ ] **Step 3: Write the full subcommands implementation**
  Replace the contents of `examples/git_subcommands.rs` with the custom `CliCommand` trait implementation:
  ```rust
  use eshu::{Cli, CliCommand, CliFlag, StoreSyntax, StoreType};
  use std::rc::Rc;

  struct AddCommand {
      flags: Vec<CliFlag>,
  }

  impl AddCommand {
      fn new() -> Result<Self, Box<dyn std::error::Error>> {
          let dry_run = CliFlag::new("dry-run")
              .with_flag_char('n')
              .with_about("Perform a dry run", "Show files that would be added without actually doing it.")
              .build()?;
          Ok(Self { flags: vec![dry_run] })
      }
  }

  impl CliCommand<'static> for AddCommand {
      fn name(&self) -> String {
          "add".to_string()
      }

      fn short_about(&self) -> String {
          "Add file contents to the index".to_string()
      }

      fn long_about(&self) -> String {
          "Add files or patterns to the staging area to prepare them for commit.".to_string()
      }

      fn flags(&self) -> &Vec<CliFlag> {
          &self.flags
      }

      fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'static>>> {
          vec![]
      }

      fn execute(&self, cli: &Cli<'static>) {
          let positionals = cli.get_stray_positional_args();
          let dry_run = cli.is_flag_entered("dry-run");

          if dry_run {
              println!("Dry-run: Preparing to add files: {:?}", positionals);
          } else {
              println!("Successfully added files: {:?}", positionals);
          }
      }
  }

  struct CommitCommand {
      flags: Vec<CliFlag>,
  }

  impl CommitCommand {
      fn new() -> Result<Self, Box<dyn std::error::Error>> {
          let message = CliFlag::new("message")
              .with_flag_char('m')
              .with_about("Commit message", "Use the given message as the commit message.")
              .with_required_store(StoreType::Value, StoreSyntax::Detached)
              .build()?;
          Ok(Self { flags: vec![message] })
      }
  }

  impl CliCommand<'static> for CommitCommand {
      fn name(&self) -> String {
          "commit".to_string()
      }

      fn short_about(&self) -> String {
          "Record changes to the repository".to_string()
      }

      fn long_about(&self) -> String {
          "Create a new commit containing the current contents of the index and the log message.".to_string()
      }

      fn flags(&self) -> &Vec<CliFlag> {
          &self.flags
      }

      fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'static>>> {
          vec![]
      }

      fn execute(&self, cli: &Cli<'static>) {
          if let Some(store) = cli.get_flag_store("message") {
              if let Some(msgs) = store.as_value() {
                  if let Some(m) = msgs.first() {
                      println!("[master] Committed changes: '{}'", m);
                      return;
                  }
              }
          }
          println!("Error: Commit message required.");
      }
  }

  fn main() -> Result<(), Box<dyn std::error::Error>> {
      let add_cmd = Rc::new(AddCommand::new()?);
      let commit_cmd = Rc::new(CommitCommand::new()?);

      let cli = Cli::new("mini-git")
          .with_version(env!("CARGO_PKG_VERSION"))
          .with_about("A minimal git subcommand simulation using eshu")
          .add_command(add_cmd)
          .add_command(commit_cmd)
          .parse();

      if !cli.is_subcommand_entered("add") && !cli.is_subcommand_entered("commit") {
          cli.print_help();
      }

      Ok(())
  }
  ```

- [ ] **Step 4: Verify the subcommands example execution**
  Run: `cargo run --example git_subcommands -- add -n src/lib.rs`
  Expected:
  ```
  Dry-run: Preparing to add files: ["src/lib.rs"]
  ```

  Run: `cargo run --example git_subcommands -- commit -m "feat: first commit"`
  Expected:
  ```
  [master] Committed changes: 'feat: first commit'
  ```

- [ ] **Step 5: Commit changes**
  ```bash
  git add examples/git_subcommands.rs
  git commit -m "add(examples): implement subcommand cli example"
  ```

---

### Task 3: Manpage Exporter Example

**Files:**
- Create: `examples/manpage.rs`

- [x] **Step 1: Write template for manpage.rs**
  Create `examples/manpage.rs` with the template:
  ```rust
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      println!("Manpage template");
      Ok(())
  }
  ```

- [x] **Step 2: Compile the example**
  Run: `cargo check --example manpage`
  Expected: PASS

- [x] **Step 3: Write the full manpage exporter implementation**
  Replace the contents of `examples/manpage.rs` with the implementation:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

  fn main() -> Result<(), Box<dyn std::error::Error>> {
      let output_flag = CliFlag::new("output")
          .with_flag_char('o')
          .with_about("Output target file", "Specify a path where the generated ROFF file should be written.")
          .with_store(StoreType::Value, StoreSyntax::Detached)
          .build()?;

      let cli = Cli::new("man-gen")
          .with_version(env!("CARGO_PKG_VERSION"))
          .with_about("Eshu ROFF Man-page Exporter")
          .with_author_and_publish_date("Your Name <you@example.com>".to_string(), "2026-06-16".to_string())
          .add_flag(output_flag)
          .parse();

      let manpage = cli.make_manpage();

      if let Some(store) = cli.get_flag_store("output") {
          if let Some(values) = store.as_value() {
              if let Some(filepath) = values.first() {
                  std::fs::write(filepath, &manpage)?;
                  println!("ROFF man-page written successfully to {}", filepath);
                  println!("To view it, run: man -l {}", filepath);
                  return Ok(());
              }
          }
      }

      println!("{}", manpage);
      Ok(())
  }
  ```

- [x] **Step 4: Verify the manpage exporter example execution**
  Run: `cargo run --example manpage -- -o /tmp/man-gen.1`
  Expected: "ROFF man-page written successfully to /tmp/man-gen.1"

- [x] **Step 5: Commit changes**
  ```bash
  git add examples/manpage.rs
  git commit -m "add(examples): implement manpage generator example"
  ```
