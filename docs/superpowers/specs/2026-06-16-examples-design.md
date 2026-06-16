# Eshu CLI Examples Design Specification

This document details the design of the three examples to be added to the `examples/` directory of the `eshu` repository.

## 1. `examples/basic.rs`

Demonstrates basic single-command CLI flag parsing, value retrieval, and positional arguments.

### Key API Concepts Demonstrated
- Building simple flags with `CliFlag::new`.
- Storing detached values and key-values.
- Checking flag presence with `is_flag_entered`.
- Unwrapping values using `Store::as_value` and `Store::as_key_value`.
- Retrieving stray positionals with `get_stray_positional_args`.

### Implementation Outline
```rust
use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let cli = Cli::new("copy-cat")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_about("A simple file processor utility using eshu")
        .add_flag(verbose_flag)
        .add_flag(output_flag)
        .add_flag(define_flag)
        .parse();

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

    let positionals = cli.get_stray_positional_args();
    if !positionals.is_empty() {
        println!("Positional arguments: {:?}", positionals);
    }

    Ok(())
}
```

---

## 2. `examples/git_subcommands.rs`

Demonstrates subcommand construction and automatic routing/execution.

### Key API Concepts Demonstrated
- Implementing the `CliCommand<'static>` trait.
- Grouping subcommand-specific flags.
- Auto-routing execution using `Cli::parse()`.
- Accessing inner CLI structures in `execute`.

### Implementation Outline
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

---

## 3. `examples/manpage.rs`

Demonstrates manpage metadata configuration and output generation.

### Key API Concepts Demonstrated
- Adding author and publish date metadata with `with_author_and_publish_date`.
- Exporting ROFF formatting via `make_manpage`.
- Integrating file writing with standard CLI parsing.

### Implementation Outline
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
