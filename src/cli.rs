use crate::cli_cmd::{CliCommand, CliFlag, CliOption};

/// Generate a command line interface
#[derive(Debug)]
pub struct Cli<'a> {
    /// The name of the program
    name: String,
    /// The version of the program
    version: String,
    /// The description of the program
    about: String,
    /// The flags of the program
    flags: Vec<CliFlag>,
    /// The options of the program
    options: Vec<CliOption>,
    /// The commands of the program
    sub_commands: Vec<Box<dyn CliCommand<'a>>>,
}

impl<'a> Cli<'a> {
    /// Create a new command line interface
    ///
    /// Consider using `Cli::new_with_capacity` for custom capacity when writing a program with
    /// a large number of commands for fine grained control
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program
    ///
    /// # Notes
    ///
    /// The default capacity is 5 bytes for the version string (`1.2.3` is 5 ASCII characters and
    /// thus 5 bytes), 255 bytes for the about string and 16 elements for the subcommand, flags, and
    /// options vector respectively.
    /// These values are a best effort guess to cover most use cases and minimize allocations
    pub fn new(name: &str) -> Self {
        let (ver_cap, about_cap) = (5, 255);
        let (flag_cap, option_cap, cmd_cap) = (16, 16, 16);
        Cli::new_with_capacity(name, ver_cap, about_cap, flag_cap, option_cap, cmd_cap)
    }
    /// Create a new command line interface with a custom capacity
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program
    /// * `ver_cap` - The capacity of the version string (`1.2.3` is 5 ASCII characters)
    /// * `about_cap` - The capacity of the about string
    /// * `flag_cap` - The capacity of the flag vector
    /// * `option_cap` - The capacity of the option vector
    /// * `cmd_cap` - The capacity of the command vector
    pub fn new_with_capacity(
        name: &str,
        ver_cap: usize,
        about_cap: usize,
        flag_cap: usize,
        option_cap: usize,
        cmd_cap: usize,
    ) -> Self {
        Cli {
            name: name.to_string(),
            version: String::with_capacity(ver_cap),
            about: String::with_capacity(about_cap),
            flags: Vec::with_capacity(flag_cap),
            options: Vec::with_capacity(option_cap),
            sub_commands: Vec::with_capacity(cmd_cap),
        }
    }
    /// Set the name of the program
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the program
    ///
    /// # Notes
    ///
    /// This will overwrite the name of the program.
    /// In general the name is already set when using `Cli::new`
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    /// Get the name of the program
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Set the version of the program
    ///
    /// # Arguments
    ///
    /// * `version` - The version of the program; e.g. `1.2.3`
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }
    /// Get the version of the program
    pub fn version(&self) -> &str {
        &self.version
    }
    /// Add a Flag command
    pub fn flag(&mut self, flag_char: char, name: &str, about: &str) {}
    /// Check if a flag is set
    ///
    /// # Arguments
    ///
    /// * `flag_char` - The flag character
    ///
    /// # Returns
    ///
    /// True if the flag is set
    /// False if the flag is not set
    pub fn get_flag(&self, flag_char: char) -> bool {}
    /// Add a Option command
    pub fn option(&mut self, flag_char: char, name: &str, about: &str, default: &str) {}
    /// Get the default value of an option
    pub fn get_option(&self, flag_char: char) -> Option<String> {}
    /// Add a Command command
    pub fn subcommand(&mut self, cmd: Box<dyn CliCommand<'a>>) {
        self.sub_commands.push(cmd);
    }
    /// Execute and parse the command line arguments
    pub fn execute(&self) {
        for args in parsed_args_vec() {
            for cmd in &self.sub_commands {
                cmd.execute(&args);
            }
        }
    }
    /// Add the help command, using the -h flag or the --help option
    pub fn help_text(&self) {}
    /// Render the man page
    ///
    /// # Returns
    ///
    /// A ROFF formatted string
    pub fn render_man_page(&self) -> RoffString {
        todo!(
            "
        - render contents of man page in ROFF format, return finished, persistable string.
        - Add some kind of further reading on where to store / register the man page for the user"
        )
    }
}

pub type RoffString = String;

impl Default for Cli<'_> {
    fn default() -> Self {
        // A cheeky bit of self promotion if name is not set later on :-)
        Cli::new("Default Name of a Eshu powered Cli Program")
    }
}
