use std::collections::BTreeMap;

use crate::{
    EshuError,
    arg_parser::parse_args,
    cli_cmd::{CliCommand, CliFlag},
    error::EshuResult,
    utils::{RoffString, Store, contains_whitespace},
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

    /// Check if a flag was entered
    ///
    /// # Arguments
    ///
    /// * `flag_name` - The name of the flag to check for (long form, e.g. `--flag-name`)
    pub fn is_flag_entered(&self, flag_name: &str) -> bool {
        self.entered_flags.contains_key(flag_name)
    }

    /// Get the store of a flag
    ///
    /// # Arguments
    ///
    /// * `flag_name` - The name of the flag to get the store for (long form, e.g. `--flag-name`)
    pub fn get_flag_store(&self, flag_name: &str) -> Option<&Store> {
        self.entered_flags.get(flag_name).map(|(_, store)| store)
    }

    /// Get the unknown arguments
    ///
    /// Only available if `handle_unknown_args` is `true`
    pub fn get_unknown_args(&self) -> Option<&Vec<String>> {
        self.unknown_args.as_ref()
    }

    /// Create a manpage for the cli
    pub fn make_manpage(&self) -> RoffString {
        todo!("create a valid manpage for the cli")
    }

    pub(crate) fn print_help(&self) {
        let footer = format!(
            "This CLI experience is provided by Eshu, version {}. For more information, visit {}",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_HOMEPAGE")
        );
        let final_string = format!("{}\n\n{}\n\n{}", header, body, footer);
        todo!("create help message & print it")
    }

    pub(crate) fn print_version(&self) {
        // Taken 1 to 1 from `git --version`
        println!("{} version {}", self.name, self.version);
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
        let help_flag = CliFlag::new("help")
            .with_flag_char('h')
            .with_short_about("Prints help information")
            .with_long_about("Also prints this message.")
            .build();
        let version_flag = CliFlag::new("version")
            .with_short_about("Prints version information")
            .with_long_about("Prints nothing else.")
            .build();
        let flags = vec![help_flag.unwrap(), version_flag.unwrap()];
        CliBuilder {
            name: name.into(),
            version: None,
            about: String::new(),
            flags,
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
