# Design Spec: Subcommand Lifetime Safety via Rc

**Date:** 2026-06-15  
**Topic:** Transition subcommands from `Box<dyn CliCommand>` to `Rc<dyn CliCommand>` to eliminate use-after-free vulnerabilities.

---

## 1. Problem Definition
The `eshu` library currently defines subcommands using `Box<dyn CliCommand<'c>>`. During recursive command parsing:
1. A nested `Cli` representing the subcommand is constructed.
2. The sub-subcommands of the matched subcommand are borrowed temporarily.
3. An `unsafe` raw pointer cast is used to extend the lifetime of these temporary references to `'a` in order to store them inside the subcommand's `Cli` structure.
4. When `CliBuilder` is consumed and dropped at the end of `parse_args`, these references become dangling. Any subsequent accesses (e.g. generating help strings or printing help) result in undefined behavior and use-after-free access to dropped memory.

---

## 2. Proposed Solution
Migrate subcommand references from `Box<dyn CliCommand<'c>>` to shared ownership using `std::rc::Rc<dyn CliCommand<'c>>`.

This permits cheap cloning of subcommand definitions (incrementing the reference count) during the parsing phase, eliminating the need to borrow temporary subcommand definitions or use any `unsafe` lifetime manipulation.

---

## 3. Detailed Changes

### 3.1. `CliCommand` Trait Updates
Modify `src/control/cmd.rs`:
```rust
pub trait CliCommand<'c> {
    fn name(&self) -> String;
    fn short_about(&self) -> String;
    fn long_about(&self) -> String;
    fn flags(&self) -> &Vec<CliFlag>;
    fn subcommands(&self) -> Vec<std::rc::Rc<dyn CliCommand<'c>>>;
    fn execute(&self, cli: &Cli<'c>);
}
```

### 3.2. `CliCmd` Struct Updates
Modify `src/control/cmd.rs` to store and clone `Rc` subcommands:
```rust
pub struct CliCmd {
    name: String,
    short_about: String,
    long_about: String,
    flags: Vec<CliFlag>,
    subcommands: Vec<std::rc::Rc<dyn CliCommand<'static>>>,
}

pub struct CliCmdBuilder {
    name: String,
    short_about: String,
    long_about: String,
    flags: Vec<CliFlag>,
    subcommands: Vec<std::rc::Rc<dyn CliCommand<'static>>>,
}
```

### 3.3. Remove `SubCommand` Enum
Delete `SubCommand` and its custom deref traits from `src/cli/mod.rs`. Update `Cli` to store `Rc` directly:
```rust
pub struct Cli<'a> {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) about: String,
    pub(crate) flags: Vec<CliFlag>,
    pub(crate) sub_commands: Vec<std::rc::Rc<dyn CliCommand<'a>>>,
    ...
}
```

### 3.4. Update `CliBuilder` API
Update `src/cli/builder.rs`:
```rust
pub struct CliBuilder<'a> {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) about: String,
    pub(crate) flags: Vec<CliFlag>,
    pub(crate) sub_commands: Vec<std::rc::Rc<dyn CliCommand<'a>>>,
    ...
}

impl<'a> CliBuilder<'a> {
    pub fn add_command(mut self, command: std::rc::Rc<dyn CliCommand<'a> + 'static>) -> Self {
        self.sub_commands.push(command);
        self
    }
}
```

### 3.5. Safe Parsing in `parser.rs`
Update `src/arg_parser/parser.rs`:
```rust
for subcommand in execute.subcommands() {
    inner_cli = inner_cli.add_command(subcommand);
}
```

---

## 4. Verification Plan
1. **Compilation**: Ensure the codebase compiles without warnings.
2. **Unit Tests**: Update all existing tests in `tests/subcmd_tests.rs` to use `Rc::new(...)` instead of `Box::new(...)`.
3. **No Unsafe Code**: Verify that the `unsafe` block in `src/arg_parser/parser.rs` has been completely deleted.
