use std::collections::BTreeMap;

use crate::{
    EshuError,
    arg_parser::parse_args,
    cli_cmd::{CliCommand, CliFlag},
    error::EshuResult,
    utils::{Store, contains_whitespace},
};

/// Generate a command line interface
#[derive(Debug)]
pub struct Cli<'a> {
    /// The name of the program
    pub(crate) name: String,
    /// The version of the program
    pub(crate) version: String,
    /// The description of the program
    pub(crate) about: String,
    /// The flags of the program
    pub(crate) flags: Vec<CliFlag>,
    /// The commands of the program
    pub(crate) sub_commands: Vec<Box<dyn CliCommand<'a>>>,
    /// The entered flags of the program. The key is the name (long form) of the flag, the value is a tuple of the flag (index into the flags vec) and the store
    pub(crate) entered_flags: BTreeMap<String, (usize, Store)>,
    /// The unknown arguments. Always `Some` (but with a length of 0 if no unknown arguments) if `handle_unknown_args` is `true`
    pub(crate) unknown_args: Option<Vec<String>>,
}

impl<'a> Cli<'a> {
    /// Create a new command line interface via the builder
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program. May contain no whitespace and may not be empty
    ///
    /// # Returns
    ///
    /// * `CliBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::Cli;
    ///
    /// let cli = Cli::new("my-cli");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> CliBuilder<'a> {
        CliBuilder::new(name)
    }
}

pub struct CliBuilder<'a> {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) about: String,
    pub(crate) flags: Vec<CliFlag>,
    pub(crate) sub_commands: Vec<Box<dyn CliCommand<'a>>>,
    pub(crate) handle_unknown_args: bool,
}

impl<'a> CliBuilder<'a> {
    /// Create a new command line interface via the builder
    /// Consider calling via `Cli::new`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program. May contain no whitespace and may not be empty
    pub fn new<S: Into<String>>(name: S) -> CliBuilder<'a> {
        CliBuilder {
            name: name.into(),
            version: None,
            about: String::new(),
            flags: Vec::new(),
            sub_commands: Vec::new(),
            handle_unknown_args: false,
        }
    }
    /// Handle unknown arguments yourself
    ///
    /// Default behavior is to print an error message.
    /// Useful if you want to parse the arguments yourself in a different way
    pub fn handle_unknown_args(mut self) -> Self {
        self.handle_unknown_args = true;
        self
    }
    /// Add a flag to the program
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag to add
    ///
    /// # Returns
    ///
    /// * `CliBuilder`
    pub fn add_flag(mut self, flag: CliFlag) -> Self {
        self.flags.push(flag);
        self
    }
    /// Add a command to the program
    /// This is used to add subcommands (e.g. `git commit`)
    ///
    /// # Arguments
    ///
    /// * `command` - The command to add
    ///
    /// # Returns
    ///
    /// * `CliBuilder`
    pub fn add_command(mut self, command: Box<dyn CliCommand<'a>>) -> Self {
        self.sub_commands.push(command);
        self
    }
    /// Set the version of the program
    /// It is recommended to use the `env!("CARGO_PKG_VERSION")` macro
    ///
    /// # Arguments
    ///
    /// * `version` - The version of the program. May not be empty
    ///
    /// # Returns
    ///
    /// * `CliBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::Cli;
    ///
    /// let cli = Cli::new("my-cli")
    ///     .with_version(env!("CARGO_PKG_VERSION"))
    ///     .build();
    /// assert!(cli.is_err()); // Not all required fields are set
    /// ```
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }
    /// Set the about text of the program
    ///
    /// # Arguments
    ///
    /// * `about` - The about text of the program
    ///
    /// # Returns
    ///
    /// * `CliBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::Cli;
    ///
    /// let cli = Cli::new("my-cli")
    ///     .with_about("My CLI with special features")
    ///     .build();
    /// assert!(cli.is_err()); // Not all required fields are set
    /// ```
    pub fn with_about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    /// Build the command line interface, validate its fields and parse the command line arguments
    /// This will return an error if the name is invalid, the version is empty, or the about is empty
    ///
    /// Will return the `Cli` struct otherwise. This can be queried with
    /// `Cli::flag_entered("flag_name")`, see the `Cli` struct for more information
    ///
    /// # Returns
    ///
    /// * `EshuResult<Cli>`
    pub fn parse(self) -> EshuResult<Cli<'a>> {
        parse_args(self)
    }
    pub(crate) fn validate_self(&self) -> EshuResult<()> {
        if contains_whitespace(&self.name) {
            return Err(EshuError::InvalidName(
                "Name must not contain whitespace".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(EshuError::EmptyString("Name must not be empty".to_string()));
        }
        if self.version.is_none() {
            return Err(EshuError::EmptyString(
                "Version must not be empty".to_string(),
            ));
        }
        if self.flags.is_empty() && self.sub_commands.is_empty() {
            return Err(EshuError::NoFlagsOrCommands);
        }
        Ok(())
    }
}
