use std::fmt::Debug;

use crate::{Cli, CliFlag};

impl Debug for dyn CliCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliCommand")
            .field("name", &self.name())
            .field(
                "about",
                &format!("{}\n{}", self.short_about(), self.long_about()),
            )
            .field("flags", &self.flags())
            .field("subcommands", &self.subcommands())
            .finish()
    }
}

/// Define a custom command
pub trait CliCommand<'c> {
    /// The name of the command
    /// May contain no whitespace and must be unique
    fn name(&self) -> &'c str;
    /// The description of the command in short form.
    /// Should be short, one liner
    fn short_about(&self) -> &'c str;
    /// The description of the command in long form.
    /// Should be longer, may contain new lines and usage examples
    /// Should NOT contain the `short_about`, must always be used together with it
    fn long_about(&self) -> &'c str;
    /// A list of all flags defined for this command
    fn flags(&self) -> Vec<CliFlag>;
    /// A list of all subcommands defined for this command
    fn subcommands(&self) -> Vec<Box<dyn CliCommand<'c>>>;
    /// The function to execute
    ///
    /// # Parameters
    ///
    /// * `cli` - The command line interface for the command. Get via `Cli::get_subcommand_cli`
    fn execute(&self, cli: &Cli<'c>);
}
