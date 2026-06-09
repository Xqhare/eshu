use std::{collections::BTreeMap, os::fd::AsRawFd};

use crate::{
    cli::builder::CliBuilder,
    utils::{RoffString, Store},
    {CliCommand, CliFlag},
};

pub mod builder;

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

    /// Create a manpage for the cli
    pub fn make_manpage(&self) -> RoffString {
        todo!("create a valid manpage for the cli")
    }

    pub(crate) fn print_help(&self) {
        let (_, width) = athena::system::terminal_size(std::io::stdout().as_raw_fd()).unwrap();
        let header = format!("{}, Version: {}\n{}\n", self.name, self.version, self.about);
        let body = {
            let mut body = "All available flags:\n".to_string();
            for flag in &self.flags {
                let mut flag_str = "\t".to_string();
                if flag.flag_char.is_some() {
                    flag_str = format!("{} -{} \t", flag_str, flag.flag_char.unwrap());
                } else {
                    flag_str = format!("{}\t\t", flag_str);
                }
                flag_str.push_str(&flag.long_flag);
                flag_str.push_str("\t");
                flag_str.push_str("\t");
                flag_str.push_str(&flag.short_about);
                flag_str.push_str("\n");
                flag_str.push_str(&flag.long_about);
                body.push_str(&flag_str);
            }
            if self.sub_commands.len() > 0 {
                body.push_str("\n");
                body.push_str("\n");
                body.push_str("All available commands:\n");
            }
            for command in &self.sub_commands {
                body.push_str(&format!("\t{}\n", command.name()));
                body.push_str(&format!(
                    "{}\n{}\n",
                    command.short_about(),
                    command.long_about()
                ));
            }
            body
        };
        let footer = format!(
            "This CLI experience is provided by Eshu, version {}. For more information, visit {}",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_HOMEPAGE")
        );
        let final_string = format!("{}\n\n\n\n{}\n\n\n\n{}", header, body, footer);
        println!("{}", final_string);
    }

    pub(crate) fn print_version(&self) {
        // Taken 1 to 1 from `git --version`
        println!("{} version {}", self.name, self.version);
    }
}
