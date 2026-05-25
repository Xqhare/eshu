use std::fmt::Debug;

use crate::ParsedArgs;

impl Debug for dyn CliCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliCommand")
            .field("name", &self.name())
            .field("about", &self.about())
            .finish()
    }
}

/// Define a custom command
pub trait CliCommand<'c> {
    /// The name of the command
    fn name(&self) -> &'c str;
    /// The description of the command
    fn about(&self) -> &'c str;
    /// A list of subcommands
    fn subcommands(&self) -> Vec<Box<dyn CliCommand<'c>>>;
    /// The function to execute
    fn execute(&self, args: &'c ParsedArgs);
}
