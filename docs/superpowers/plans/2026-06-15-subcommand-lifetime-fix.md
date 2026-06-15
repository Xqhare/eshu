# Subcommand Lifetime Safety via Rc Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate use-after-free vulnerabilities and unsafe lifetime casts in subcommand parsing by transitioning from `Box<dyn CliCommand>` to `std::rc::Rc<dyn CliCommand>`.

**Architecture:** Use `std::rc::Rc` to share ownership of subcommands. The parser can then clone `Rc` pointers to subcommand definitions safely during recursive parsing of matched subcommand scopes. This completely removes the need for unsafe pointer casting and temporary borrows.

**Tech Stack:** Rust (standard library: `std::rc::Rc`).

---

### Task 1: Update `CliCommand` and `CliCmd` in `src/control/cmd.rs`

**Files:**
- Modify: `src/control/cmd.rs`

- [ ] **Step 1: Modify `src/control/cmd.rs`**
  Change the `CliCommand` trait's `subcommands` method, `CliCmd`'s structure, and `CliCmdBuilder`'s methods to use `std::rc::Rc` instead of `Box`.
  
  Replace lines 18, 39, 68, 80, and 98 with the following changes:
  
  ```rust
  // In `pub trait CliCommand<'c>`:
  fn subcommands(&self) -> Vec<std::rc::Rc<dyn CliCommand<'c>>>;
  
  // In `pub struct CliCmd`:
  subcommands: Vec<std::rc::Rc<dyn CliCommand<'static>>>,
  
  // In `impl CliCommand<'static> for CliCmd`:
  fn subcommands(&self) -> Vec<std::rc::Rc<dyn CliCommand<'static>>> {
      self.subcommands.clone()
  }
  
  // In `pub struct CliCmdBuilder`:
  subcommands: Vec<std::rc::Rc<dyn CliCommand<'static>>>,
  
  // In `impl CliCmdBuilder`:
  pub fn add_subcommand(mut self, subcommand: std::rc::Rc<dyn CliCommand<'static>>) -> Self {
      self.subcommands.push(subcommand);
      self
  }
  ```

- [ ] **Step 2: Run cargo check to verify compile failures in other files**
  Run: `cargo check`
  Expected: Fail due to type mismatches in `builder.rs`, `parser.rs`, and tests.

- [ ] **Step 3: Commit interface changes**
  ```bash
  git add src/control/cmd.rs
  git commit -m "refactor(control): migrate subcommand trait methods to use Rc"
  ```

---

### Task 2: Remove `SubCommand` Enum and Clean Up `src/cli/mod.rs` and `src/cli/help.rs`

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/help.rs`

- [ ] **Step 1: Modify `src/cli/mod.rs`**
  Delete the `SubCommand` enum and update `Cli<'a>` to store a vector of `Rc<dyn CliCommand<'a>>` directly:
  
  ```rust
  // Remove `SubCommand` enum completely from lines 12-25.
  
  // Update `Cli` struct in `src/cli/mod.rs`:
  pub struct Cli<'a> {
      pub(crate) name: String,
      pub(crate) version: String,
      pub(crate) about: String,
      pub(crate) flags: Vec<CliFlag>,
      pub(crate) sub_commands: Vec<std::rc::Rc<dyn CliCommand<'a>>>,
      pub(crate) entered_flags: BTreeMap<String, (usize, Store)>,
      pub(crate) unknown_args: Option<Vec<String>>,
      pub(crate) stray_positional_args: Vec<String>,
      pub(crate) sub_cmd_cli: BTreeMap<String, Cli<'a>>,
  }
  ```

- [ ] **Step 2: Modify `src/cli/help.rs`**
  Update help generation loops to dereference the subcommand via `Rc` instead of dereferencing `SubCommand`.
  
  In `src/cli/help.rs`, change lines 45-48:
  
  ```rust
  // Target content:
  for command in &cli.sub_commands {
      make_subcmd(&**command, out);
      out.push_str(SECTION_BREAK);
  }
  
  // Replacement content:
  for command in &cli.sub_commands {
      make_subcmd(command.as_ref(), out);
      out.push_str(SECTION_BREAK);
  }
  ```

- [ ] **Step 3: Commit changes**
  ```bash
  git add src/cli/mod.rs src/cli/help.rs
  git commit -m "refactor(cli): remove SubCommand enum wrapper and use Rc directly"
  ```

---

### Task 3: Update `CliBuilder` in `src/cli/builder.rs`

**Files:**
- Modify: `src/cli/builder.rs`

- [ ] **Step 1: Modify `src/cli/builder.rs`**
  Change the type of `sub_commands` in `CliBuilder<'a>` to hold `Rc` pointers, and update `add_command` while removing `add_command_ref`.
  
  Replace lines 14, 95-102:
  
  ```rust
  // In `pub struct CliBuilder<'a>`:
  pub(crate) sub_commands: Vec<std::rc::Rc<dyn CliCommand<'a>>>,
  
  // In `impl<'a> CliBuilder<'a>`:
  pub fn add_command(mut self, command: std::rc::Rc<dyn CliCommand<'a> + 'static>) -> Self {
      self.sub_commands.push(command);
      self
  }
  // Remove `add_command_ref` method completely.
  ```

- [ ] **Step 2: Commit builder changes**
  ```bash
  git add src/cli/builder.rs
  git commit -m "refactor(cli): update CliBuilder to accept Rc subcommands"
  ```

---

### Task 4: Remove Unsafe Casting in `src/arg_parser/parser.rs`

**Files:**
- Modify: `src/arg_parser/parser.rs`

- [ ] **Step 1: Modify `src/arg_parser/parser.rs`**
  Eliminate the `unsafe` block in `parse_subcommand`. Clone the subcommand's `Rc` directly and pass it to `add_command`.
  
  Replace lines 221-225 in `src/arg_parser/parser.rs`:
  
  ```rust
  // Target content:
  for subcommand in execute.subcommands() {
      let cmd_ref = subcommand.as_ref();
      let cmd_ref_long: &'a dyn CliCommand<'a> = unsafe { &*(cmd_ref as *const dyn CliCommand<'a>) };
      inner_cli = inner_cli.add_command_ref(cmd_ref_long);
  }
  
  // Replacement content:
  for subcommand in execute.subcommands() {
      inner_cli = inner_cli.add_command(subcommand.clone());
  }
  ```

- [ ] **Step 2: Commit parser changes**
  ```bash
  git add src/arg_parser/parser.rs
  git commit -m "fix(arg_parser): remove unsafe block and use Rc cloning for subcommands"
  ```

---

### Task 5: Update Tests and Verify Compilation

**Files:**
- Modify: `tests/subcmd_tests.rs`

- [ ] **Step 1: Modify `tests/subcmd_tests.rs`**
  Update the tests to register subcommands using `std::rc::Rc::new` instead of `Box::new`.
  
  Replace `Box::new` with `std::rc::Rc::new` at lines 7 and 38 in `tests/subcmd_tests.rs`:
  
  ```rust
  // In `tests/subcmd_tests.rs`:
  // Line 7:
  .add_command(std::rc::Rc::new(
  
  // Line 38:
  .add_command(std::rc::Rc::new(
  ```

- [ ] **Step 2: Run all tests to verify safety and correctness**
  Run: `cargo test`
  Expected: All tests pass successfully, and cargo compiles without any unused warnings or errors.

- [ ] **Step 3: Commit test updates**
  ```bash
  git add tests/subcmd_tests.rs
  git commit -m "test(subcmd): update test suite to use Rc for subcommands"
  ```
