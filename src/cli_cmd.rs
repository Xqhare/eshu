use std::fmt::Debug;

use crate::ParsedArgs;

impl Debug for dyn CliCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliCommand")
            .field("name", &self.name())
            .field("about", &self.about())
            .field("flags", &self.flags())
            .field("options", &self.options())
            .field("subcommands", &self.subcommands())
            .finish()
    }
}

/// Define a custom command
pub trait CliCommand<'c> {
    /// The name of the command
    fn name(&self) -> &'c str;
    /// The description of the command
    fn about(&self) -> &'c str;
    /// A list of flags
    fn flags(&self) -> Vec<CliFlag>;
    /// A list of options
    fn options(&self) -> Vec<CliOption>;
    /// A list of subcommands
    fn subcommands(&self) -> Vec<Box<dyn CliCommand<'c>>>;
    /// The function to execute
    fn execute(&self, args: &'c ParsedArgs);
}

/// Defines a flag
pub struct CliFlag {
    flag_char: char,
    name: String,
    about: String,
}

impl Debug for CliFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliFlag")
            .field("flag_char", &self.flag_char)
            .field("name", &self.name)
            .field("about", &self.about)
            .finish()
    }
}

/// Defines an option
pub struct CliOption {
    flag_char: char,
    name: String,
    about: String,
    default: Option<String>,
}

impl Debug for CliOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliOption")
            .field("flag_char", &self.flag_char)
            .field("name", &self.name)
            .field("about", &self.about)
            .field("default", &self.default)
            .finish()
    }
}
