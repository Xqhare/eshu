use std::{fmt::Debug, rc::Rc};

use crate::{
    Cli, CliFlag,
    error::{EshuErrorKind, EshuResult},
};
use nemesis::NemesisError;

impl Debug for dyn CliCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliCommand")
            .field("name", &self.name())
            .field("short_about", &self.short_about())
            .field("long_about", &self.long_about())
            .field("flags", &self.flags())
            .field("subcommands", &self.subcommands())
            .field("execute", &"function")
            .finish()
    }
}

/// Define a custom command
pub trait CliCommand<'c> {
    /// The name of the command
    /// May contain no whitespace and must be unique
    fn name(&self) -> String;
    /// The description of the command in short form.
    /// Should be short, one liner
    fn short_about(&self) -> String;
    /// The description of the command in long form.
    /// Should be longer, may contain new lines and usage examples
    /// Should NOT contain the `short_about`, must always be used together with it
    fn long_about(&self) -> String;
    /// A list of all flags defined for this command
    fn flags(&self) -> &Vec<CliFlag>;
    /// A list of all subcommands defined for this command
    fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'c>>>;
    /// The function to execute
    ///
    /// # Parameters
    ///
    /// * `cli` - The command line interface for the command. Get via `Cli::get_subcommand_cli`
    fn execute(&self, cli: &Cli<'c>);
}

/// A custom command
///
/// Use this only if you want to create a subcommand with only flags or subcommands.
/// This struct does not provide the `execute` function.
/// If you want that, please implement the `CliCommand` trait yourself.
///
/// This struct is intended for simple flag grouping and subcommand grouping and internal testing
pub struct CliCmd {
    name: String,
    short_about: String,
    long_about: String,
    flags: Vec<CliFlag>,
    subcommands: Vec<Rc<dyn CliCommand<'static>>>,
}

impl CliCmd {
    /// Create a new command
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the command. May contain no whitespace and must be unique
    ///
    /// # Returns
    ///
    /// * `CliCmdBuilder` - The builder for the command
    ///
    /// # Example
    ///
    /// ```
    /// use eshu::CliCmd;
    ///
    /// let cmd = CliCmd::new("cmd-name");
    /// ```
    #[inline]
    #[must_use]
    #[expect(clippy::new_ret_no_self, reason = "Builder pattern")]
    pub fn new(name: &str) -> CliCmdBuilder {
        CliCmdBuilder {
            name: name.to_string(),
            short_about: String::new(),
            long_about: String::new(),
            flags: Vec::new(),
            subcommands: Vec::new(),
        }
    }
}

impl CliCommand<'static> for CliCmd {
    #[inline]
    fn name(&self) -> String {
        self.name.clone()
    }
    #[inline]
    fn short_about(&self) -> String {
        self.short_about.clone()
    }
    #[inline]
    fn long_about(&self) -> String {
        self.long_about.clone()
    }
    #[inline]
    fn flags(&self) -> &Vec<CliFlag> {
        &self.flags
    }
    #[inline]
    fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'static>>> {
        self.subcommands.clone()
    }
    #[inline]
    fn execute(&self, _cli: &Cli<'_>) {}
}

/// Builder for `CliCmd`
pub struct CliCmdBuilder {
    name: String,
    short_about: String,
    long_about: String,
    flags: Vec<CliFlag>,
    subcommands: Vec<Rc<dyn CliCommand<'static>>>,
}

impl CliCmdBuilder {
    /// Set the about text
    pub fn with_about(mut self, short_about: &str, long_about: &str) -> Self {
        self.short_about = short_about.to_string();
        self.long_about = long_about.to_string();
        self
    }

    /// Add a flag to the command
    pub fn add_flag(mut self, flag: CliFlag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Add a subcommand to the command
    pub fn add_subcommand(mut self, subcommand: Rc<dyn CliCommand<'static>>) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    /// Build the command
    ///
    /// # Errors
    /// If the name or about is empty or if no flags or subcommands are defined
    pub fn build(self) -> EshuResult<CliCmd> {
        if self.name.is_empty() {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::EmptyString("Command name must not be empty".to_string()),
            ));
        }
        if self.short_about.is_empty() {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::EmptyString("Command description must not be empty".to_string()),
            ));
        }
        if self.flags.is_empty() && self.subcommands.is_empty() {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::NoFlagsOrCommands,
            ));
        }
        Ok(CliCmd {
            name: self.name,
            short_about: self.short_about,
            long_about: self.long_about,
            flags: self.flags,
            subcommands: self.subcommands,
        })
    }
}
