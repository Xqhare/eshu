use crate::{
    Cli, CliCommand, CliFlag, EshuError,
    arg_parser::parse_args,
    error::EshuResult,
    utils::{contains_whitespace, get_params_make_args},
};

pub struct CliBuilder<'a> {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) about: String,
    pub(crate) flags: Vec<CliFlag>,
    pub(crate) sub_commands: Vec<Box<dyn CliCommand<'a>>>,
    pub(crate) handle_unknown_args: bool,
    basic: bool,
    pub(crate) auto_execution: bool,
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
            basic: false,
            auto_execution: true,
        }
    }
    /// Disables the auto execution of subcommands
    ///
    /// Disabling auto execution will require the user to manually call `CliCommand::execute`.
    /// If enabled (default behavior), `CliCommand::execute` will be called automatically
    /// called when the subcommand is encountered during parsing
    pub fn without_auto_execution(mut self) -> Self {
        self.auto_execution = false;
        self
    }
    /// Removes the requirement of having more flags than just the provided `help` and
    /// `version` ones.
    pub fn basic(mut self) -> Self {
        self.basic = true;
        self
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
    ///     .handle_unknown_args()
    ///     .basic()
    ///     .parse();
    /// assert!(cli.is_ok());
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
    ///     .parse();
    /// assert!(cli.is_err()); // Not all required fields are set
    /// ```
    pub fn with_about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    /// Build the command line interface, validate its fields and get & parse the command line arguments passed into the program
    /// This will return an error if the name is invalid, the version is empty, or the about is empty
    ///
    /// Will return the `Cli` struct otherwise. This can be queried with
    /// `Cli::flag_entered("flag_name")`, see the `Cli` struct for more information
    ///
    /// # Returns
    ///
    /// * `EshuResult<Cli>`
    pub fn parse(self) -> EshuResult<Cli<'a>> {
        parse_args(self, get_params_make_args())
    }
    /// Build the command line interface, validate its fields and parse the command line arguments
    ///
    /// Use this if you want to get the command line arguments yourself (`Eshu` uses `std::env::args_os()` and lossy converts them into `String`)
    ///
    /// # Arguments
    ///
    /// * `params` - The command line arguments
    ///
    /// # Returns
    ///
    /// * `EshuResult<Cli>`
    ///
    /// # Note
    ///
    /// `Eshu` expects the very first element of the passed in `Vec<String>` to be the program name. This element is skipped and can be anything, it must be present however.
    pub fn parse_custom(self, params: Vec<String>) -> EshuResult<Cli<'a>> {
        parse_args(self, params.into_iter().skip(1).collect())
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
        // `help` and `version` are GNU required; they are always present
        // Having no other flags or commands seems like a developer error
        if self.flags.len() == 2 && self.sub_commands.is_empty() && !self.basic {
            return Err(EshuError::NoFlagsOrCommands);
        }
        Ok(())
    }
}
