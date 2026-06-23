use std::{collections::HashSet, process::exit, rc::Rc};

use crate::{
    Cli, CliCommand, CliFlag, EshuErrorKind,
    arg_parser::parse_args,
    error::EshuResult,
    utils::{contains_whitespace, get_params_make_args},
};
use nemesis::NemesisError;

/// A builder for constructing and parsing a [`Cli`].
#[expect(clippy::module_name_repetitions, reason = "Builder pattern")]
pub struct CliBuilder<'a> {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) about: String,
    pub(crate) flags: Vec<CliFlag>,
    pub(crate) sub_commands: Vec<Rc<dyn CliCommand<'a>>>,
    pub(crate) handle_unknown_args: bool,
    basic: bool,
    pub(crate) auto_execution: bool,
    pub(crate) author: String,
    pub(crate) publish_date: String,
}

impl<'a> CliBuilder<'a> {
    /// Create a new command line interface via the builder
    /// Consider calling via `Cli::new`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program. May contain no whitespace and may not be empty
    #[expect(clippy::expect_used, reason = "Manual checked and constructed")]
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
        let flags = vec![
            help_flag.expect("Manual checked, constructed above"),
            version_flag.expect("Manual checked, constructed above"),
        ];
        CliBuilder {
            name: name.into(),
            version: None,
            about: String::new(),
            flags,
            sub_commands: Vec::new(),
            handle_unknown_args: false,
            basic: false,
            auto_execution: true,
            author: String::new(),
            publish_date: String::new(),
        }
    }
    /// Sets the author and publish date for the man page
    ///
    /// # Note
    /// Only relevant for man pages. Not relevant for help output.
    pub fn with_author_and_publish_date(mut self, author: String, publish_date: String) -> Self {
        self.author = author;
        self.publish_date = publish_date;
        self
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
    pub fn add_command(mut self, command: Rc<dyn CliCommand<'a> + 'static>) -> Self {
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
    ///     .try_parse();
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
    ///     .try_parse();
    /// assert!(cli.is_err()); // Not all required fields are set
    /// ```
    pub fn with_about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    /// Build the command line interface, validate its fields and get & parse the command line arguments passed into the program
    /// This will return an error if the name is invalid, the version is empty, or the about is empty
    ///
    /// If validation or parsing fails, this will print the error using `eprintln!` and exit with code 1.
    ///
    /// Will return the `Cli` struct otherwise. This can be queried with
    /// `Cli::flag_entered("flag_name")`, see the `Cli` struct for more information
    ///
    /// # Returns
    ///
    /// * `Cli`
    #[expect(clippy::print_stderr, reason = "Need to print to stderr")]
    pub fn parse(self) -> Cli<'a> {
        match self.try_parse() {
            Ok(cli) => cli,
            Err(err) => {
                eprintln!("{err}");
                exit(1);
            }
        }
    }
    /// Build the command line interface, validate its fields and parse the command line arguments
    ///
    /// Use this if you want to get the command line arguments yourself (`Eshu` uses `std::env::args_os()` and lossy converts them into `String`)
    ///
    /// If validation or parsing fails, this will print the error using `eprintln!` and exit with code 1.
    ///
    /// # Arguments
    ///
    /// * `params` - The command line arguments
    ///
    /// # Returns
    ///
    /// * `Cli`
    ///
    #[expect(clippy::print_stderr, reason = "Need to print to stderr")]
    pub fn parse_custom(self, params: Vec<String>) -> Cli<'a> {
        match self.try_parse_custom(params) {
            Ok(cli) => cli,
            Err(err) => {
                eprintln!("{err}");
                exit(1);
            }
        }
    }
    /// Non-exiting version of parse. Bubbles up any validation or parsing errors.
    pub fn try_parse(self) -> EshuResult<Cli<'a>> {
        self.try_parse_custom(get_params_make_args())
    }

    /// Non-exiting custom version of parse.
    pub fn try_parse_custom(self, params: Vec<String>) -> EshuResult<Cli<'a>> {
        parse_args(self, params)
    }
    pub(crate) fn validate_self(&self) -> EshuResult<()> {
        if contains_whitespace(&self.name) {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::InvalidName("Name must not contain whitespace".to_string()),
            ));
        }
        if self.name.is_empty() {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::EmptyString("Name must not be empty".to_string()),
            ));
        }
        if self.version.is_none() {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::EmptyString("Version must not be empty".to_string()),
            ));
        }
        // `help` and `version` are GNU required; they are always present
        // Having no other flags or commands seems like a developer error
        if self.flags.len() == 2 && self.sub_commands.is_empty() && !self.basic {
            return Err(NemesisError::new(
                "eshu::builder",
                EshuErrorKind::NoFlagsOrCommands,
            ).add_ctx("Should you intend to provide no flags or commands, use the `basic` method on the `CliBuilder`."));
        }
        let mut flag_names = HashSet::new();
        let mut flag_chars = HashSet::new();
        for flag in &self.flags {
            if !flag_names.insert(&flag.long_flag) {
                return Err(NemesisError::new(
                    "eshu::builder",
                    EshuErrorKind::Duplicate(format!("Duplicate Flag: {}", flag.long_flag)),
                ));
            }
            if let Some(c) = flag.flag_char {
                let duplicate = !flag_chars.insert(c);
                if duplicate {
                    return Err(NemesisError::new(
                        "eshu::builder",
                        EshuErrorKind::Duplicate(format!("Duplicate Short Flag Char: {c}")),
                    ));
                }
            }
        }
        Ok(())
    }
}
