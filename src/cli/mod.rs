use std::collections::BTreeMap;

use crate::{
    CliCommand, CliFlag,
    cli::{builder::CliBuilder, help::help},
    utils::{RoffString, Store},
};

pub mod builder;
mod help;

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
    /// Positional arguments without a corresponding flag
    pub(crate) stray_positional_args: Vec<String>,
    /// The cli of the subcommands for each subcommand called
    pub(crate) sub_cmd_cli: BTreeMap<String, Cli<'a>>,
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

    /// Get the stray positional arguments
    ///
    /// Stray positional arguments are positional arguments without a corresponding flag.
    ///
    /// # Returns
    ///
    /// * `&Vec<String>`
    pub fn get_stray_positional_args(&self) -> &Vec<String> {
        &self.stray_positional_args
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

    /// Check if a subcommand was entered
    pub fn is_subcommand_entered(&self, subcommand_name: &str) -> bool {
        self.sub_cmd_cli.contains_key(subcommand_name)
    }

    /// Get the cli of a subcommand
    ///
    /// Will only return `Some` if the subcommand was entered
    pub fn get_subcommand_cli(&self, subcommand_name: &str) -> Option<&Cli<'a>> {
        self.sub_cmd_cli.get(subcommand_name)
    }

    /// Create a manpage for the cli
    ///
    /// The returned string is a valid manpage in `roff` format
    pub fn make_manpage(&self) -> RoffString {
        todo!("create a valid manpage for the cli - complex, do last")
    }

    /// Print the help to stdout
    pub(crate) fn print_help(&self) {
        println!("{}", help(self));
    }

    /// Print the version to stdout
    pub(crate) fn print_version(&self) {
        // Taken 1 to 1 from `git --version`
        println!("{} version {}", self.name, self.version);
    }
}
