use std::fmt::Debug;

use crate::{
    EshuError,
    error::EshuResult,
    utils::{contains_whitespace, starts_with_dash},
};

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
    /// * `args` - All command line arguments immediately to the right of the subcommand
    fn execute(&self, args: &Vec<String>);
}

/// The kind of the store
///
/// * `Optional` - No value is required with the flag, but it may be passed with one
/// * `Required` - A value is required with the flag
#[derive(Debug, Clone, Copy)]
pub enum StoreKind {
    /// A value is optional
    Optional,
    /// A value is required
    Required,
}

/// The type of the store
///
/// * `Value` - A value (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed values.)
/// * `KeyValue` - A key-value pair (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed key-value pairs.)
#[derive(Debug, Clone, Copy)]
pub enum StoreType {
    /// A value
    Value,
    /// A key-value pair
    KeyValue,
}

/// The syntax of the store
///
/// * `Attached` - The flag is attached to the command. (Between the flag and value must be an equal sign `=`)
/// * `Detached` - The flag is detached from the command. (Between the flag and value must be a space ` `)
///
/// # Usage
///
/// If a flag is attached, the calling syntax will be `--flag-name=value`.
/// If the flag has a required store, the calling syntax will be `--flag-name=value`, otherwise the calling syntax will be `--flag-name[=value]`.
///
/// If a flag is detached, the calling syntax will be `--flag-name value`.
/// If the flag has a required store, the calling syntax will be `--flag-name value`, otherwise the calling syntax will be `--flag-name [value]`.
/// Please note, this also allows `--flag-name "a value with spaces"`, which is not possible with an attached flag (blame POSIX).
#[derive(Debug, Clone, Copy)]
pub enum StoreSyntax {
    /// Attached to the flag; `--flag-name=value` or `--flag-name=key=value`
    Attached,
    /// Detached from the flag; `--flag-name value` or `--flag-name "a value with spaces"` or `--flag-name key=value`
    Detached,
}

/// Defines a flag
/// Is used to parse command line arguments and hold metadata
pub struct CliFlag {
    /// The, optional, short flag, e.g. `-f`
    /// May not contain a dash, `-`
    pub(crate) flag_char: Option<char>,
    /// The long flag, e.g. `--flag-name`
    /// This is also used as the name of the flag
    /// May contain no whitespace or a leading dash or double dash, `-` or `--`
    pub(crate) long_flag: String,
    /// The about or help text for the flag.
    /// May be used on its own or together with `long_about`
    /// Should be short, one liner
    pub(crate) short_about: String,
    /// The long about or help text for the flag.
    /// Will be always used together with `short_about` if used at all.
    /// Should be longer, may contain new lines and usage examples
    pub(crate) long_about: String,
    /// Does the flag require passed arguments
    pub(crate) required_store: bool,
    /// Does the flag accept passed arguments
    pub(crate) storing: bool,
    /// The type of the store (Value or KeyValue)
    pub(crate) store_type: Option<StoreType>,
    /// The syntax of the store (Attached or Detached)
    pub(crate) store_syntax: Option<StoreSyntax>,
}

impl CliFlag {
    /// Create a new flag using the builder
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the flag, also used as the long flag. May contain no whitespace or a leading dash or double dash, `-` or `--`
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    ///
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name");
    /// ```
    pub fn new(name: &str) -> CliFlagBuilder {
        CliFlagBuilder::new(name)
    }
    /// Create a new custom flag
    /// Consider using the builder instead via `new`
    ///
    /// # Note
    ///
    /// This function will validate the input parameters and return an error if invalid
    ///
    /// # Parameters
    ///
    /// * `flag_char` - The, optional, short flag, e.g. `-f` (may not contain a dash, `-`).
    /// * `long_flag` - The long flag, e.g. `--flag-name` (may contain no whitespace or a leading dash or double dash, `-` or `--`).
    /// * `short_about` - The about or help text for the flag. (May contain no newlines, may be used on its own or together with `long_about`).
    /// * `long_about` - The long about or help text for the flag. (Will be always used together with `short_about` if used at all).
    /// * `required_store` - Does the flag require passed arguments (If set to true, `storing` must also be set to true).
    /// * `storing` - Does the flag accept passed arguments (Only setting this to true means that the flag accepts optional arguments).
    /// * `store_type` - The type of the store (Value or KeyValue). Only relevant if `storing` is set to true.
    /// * `store_syntax` - The syntax of the store (Attached or Detached). Only relevant if `storing` is set to true.
    ///
    /// # Returns
    ///
    /// * `EshuResult<CliFlag>`
    ///
    /// # Example
    ///
    /// ```
    /// use eshu::{CliFlag, StoreType, StoreSyntax};
    ///
    /// let flag = CliFlag::new_custom(
    ///     Some('l'),
    ///     "long-flag".to_string(),
    ///     "short about".to_string(),
    ///     "long about".to_string(),
    ///     false,
    ///     true,
    ///     Some(StoreType::Value),
    ///     Some(StoreSyntax::Attached)
    /// );
    /// assert!(flag.is_ok());
    /// ```
    pub fn new_custom(
        flag_char: Option<char>,
        long_flag: String,
        short_about: String,
        long_about: String,
        required_store: bool,
        storing: bool,
        store_type: Option<StoreType>,
        store_syntax: Option<StoreSyntax>,
    ) -> EshuResult<CliFlag> {
        if long_flag.is_empty() {
            return Err(EshuError::EmptyString(
                "Flag name must not be empty".to_string(),
            ));
        }
        if required_store && !storing {
            return Err(EshuError::Storage(
                "Flags that require arguments must accept them".to_string(),
            ));
        }
        if let Some(char) = flag_char {
            if !char.is_ascii_alphabetic() {
                return Err(EshuError::InvalidName(
                    "Short flag must be a letter".to_string(),
                ));
            }
        }
        if starts_with_dash(&long_flag) {
            return Err(EshuError::InvalidName(
                "Flag name must not start with a dash or double dash".to_string(),
            ));
        }
        if contains_whitespace(&long_flag) {
            return Err(EshuError::InvalidName(
                "Flag name must not contain whitespace".to_string(),
            ));
        }
        if storing && (store_syntax.is_none() || store_type.is_none()) {
            return Err(EshuError::Storage(
                    "Flags that accept arguments must have a store type, store kind and store syntax set".to_string(),
                ));
        }
        Ok(CliFlag {
            flag_char,
            long_flag,
            short_about,
            long_about,
            required_store,
            storing,
            store_type,
            store_syntax,
        })
    }
}

// This doc really is just to make clippy shut up. The real doc is on `CliFlag`
/// Builder for a `CliFlag`
/// Consider calling the builder instead via `CliFlag::new`
#[derive(Debug)]
pub struct CliFlagBuilder {
    /// The flag character
    flag_char: Option<char>,
    /// The long flag or name
    long_flag: String,
    /// The short about or help text
    short_about: String,
    /// The long about or help text
    long_about: String,
    /// Does the flag require passed arguments
    required_store: bool,
    /// Does the flag accept passed arguments
    storing: bool,
    /// The type of the store
    store_type: Option<StoreType>,
    /// The syntax of the store
    store_syntax: Option<StoreSyntax>,
}

impl CliFlagBuilder {
    /// Create a new flag using the builder
    /// Consider calling via `CliFlag::new`
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the flag, also used as the long flag. May contain no whitespace or a leading dash or double dash, `-` or `--`
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name");
    /// ```
    pub fn new(name: &str) -> CliFlagBuilder {
        CliFlagBuilder {
            flag_char: None,
            long_flag: name.to_string(),
            short_about: String::new(),
            long_about: String::new(),
            required_store: false,
            storing: false,
            store_type: None,
            store_syntax: None,
        }
    }
    /// Add the optional short flag character, e.g. `-f`
    /// Char must be unique and valid Unicode
    ///
    /// # Parameters
    ///
    /// * `flag_char` - The flag character
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_flag_char('-')
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_flag_char(mut self, flag_char: char) -> CliFlagBuilder {
        self.flag_char = Some(flag_char);
        self
    }
    /// Change the long flag or name, e.g. `--flag-name`
    ///
    /// # Note
    ///
    /// While the flag name may contain no whitespace or a leading dash or double dash, `-` or `--`
    /// this is not checked by this function. The check is done in `CliFlag::build`
    ///
    /// # Parameters
    ///
    /// * `long_flag` - The long flag
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .change_long_flag("new-flag-name")
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn change_long_flag(mut self, long_flag: &str) -> CliFlagBuilder {
        self.long_flag = long_flag.to_string();
        self
    }
    /// Change the short about text
    ///
    /// # Parameters
    ///
    /// * `short_about` - The short about text
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_short_about("short about")
    ///     .build(); // build ensures the entire flag built is valid (missing long about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_short_about(mut self, short_about: &str) -> CliFlagBuilder {
        self.short_about = short_about.to_string();
        self
    }
    /// Change the long about text
    ///
    /// # Parameters
    ///
    /// * `long_about` - The long about text
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_long_about("long about")
    ///     .build(); // build ensures the entire flag built is valid (missing short about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_long_about(mut self, long_about: &str) -> CliFlagBuilder {
        self.long_about = long_about.to_string();
        self
    }
    /// Mark the flag as storing required arguments
    /// This is for flags that must have arguments, e.g. `-f arg`
    /// Calling this will also mark the flag as storing (see `with_storing`)
    /// Use this function if the flag requires arguments
    ///
    /// # Note
    ///
    /// This function must be used together with `with_store`
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_required_store()
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_required_store(mut self) -> CliFlagBuilder {
        self.storing = true;
        self.required_store = true;
        self
    }
    /// Mark the flag as storing, meaning it accepts arguments (but does not require them)
    /// If a flag is built by calling `with_required_store`, calling this function is superfluous.
    /// Use this function if the flag accepts arguments optionally
    ///
    /// # Note
    ///
    /// This function must be used together with `with_store`
    ///
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_storing()
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_storing(mut self) -> CliFlagBuilder {
        self.storing = true;
        self
    }
    /// Defines the store type, store kind and store syntax of the flag
    /// Required if `storing` is set to true
    /// Always use this function if the flag accepts arguments
    ///
    /// # Note
    ///
    /// This function must be used together with `with_storing` or `with_required_store`
    ///
    /// # Parameters
    ///
    /// * `store_type` - The store type of the flag.
    ///     - `StoreType::Value` - The flag accepts a single value.
    ///     - `StoreType::KeyValue` - The flag accepts a key-value pair.
    /// * `store_syntax` - The store syntax of the flag.
    ///     - `StoreSyntax::Attached` - The flag is attached to the command. (Between the flag and
    ///     value must be an equal sign `=`)
    ///     - `StoreSyntax::Detached` - The flag is detached from the command. (The value must be
    ///     immediately to the right of the flag. Use this if you want the End-Of-Flags marker `--` to pass all following arguments as values into the store of this flag)
    ///     
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::{CliFlag, StoreType, StoreSyntax};
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_storing()
    ///     .with_store(StoreType::Value, StoreSyntax::Attached)
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_store(
        mut self,
        store_type: StoreType,
        store_syntax: StoreSyntax,
    ) -> CliFlagBuilder {
        self.store_type = Some(store_type);
        self.store_syntax = Some(store_syntax);
        // Doc explicitly states that calling `with_storing` is required;
        // Still set this to true to be nice, developer intent is clear.
        self.storing = true;
        self
    }
    /// Build the flag
    /// This will check if all the parameters are valid and then build the flag
    /// If any of the parameters are invalid, an error will be returned
    ///
    /// # Returns
    ///
    /// * `EshuResult<CliFlag>`
    ///
    /// # Example
    /// ```
    /// use eshu::CliFlag;
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_short_about("short about")
    ///     .with_long_about("long about \n with new line")
    ///     .build();
    /// assert!(flag.is_ok());
    /// ```
    pub fn build(mut self) -> EshuResult<CliFlag> {
        if self.required_store && !self.storing {
            return Err(EshuError::Storage(
                "Flags that require arguments must accept them".to_string(),
            ));
        }
        if let Some(char) = self.flag_char {
            if !char.is_ascii_alphabetic() {
                return Err(EshuError::InvalidName("Flag must be a letter".to_string()));
            }
        }
        if self.long_flag.is_empty() {
            return Err(EshuError::EmptyString(
                "Flag name must not be empty".to_string(),
            ));
        }
        if starts_with_dash(&self.long_flag) {
            // Lets be nice, developer intent is clear; but only if -- not -
            if let Some(removed_dashes) = self.long_flag.strip_prefix("--") {
                self.long_flag = removed_dashes.to_string();
            } else {
                return Err(EshuError::InvalidName(format!(
                    "Flag name must not start with a dash. Got: {}",
                    self.long_flag
                )));
            }
        }
        if contains_whitespace(&self.long_flag) {
            return Err(EshuError::InvalidName(
                "Flag name must not contain whitespace".to_string(),
            ));
        }
        if self.short_about.is_empty() || self.long_about.is_empty() {
            return Err(EshuError::EmptyString(
                "Short and long about must not be empty".to_string(),
            ));
        }
        if self.storing && (self.store_syntax.is_none() || self.store_type.is_none()) {
            return Err(EshuError::Storage(
                    "Flags that accept arguments must have a store type, store kind and store syntax set".to_string(),
                ));
        }
        Ok(CliFlag {
            flag_char: self.flag_char,
            long_flag: self.long_flag,
            short_about: self.short_about,
            long_about: self.long_about,
            required_store: self.required_store,
            storing: self.storing,
            store_type: self.store_type,
            store_syntax: self.store_syntax,
        })
    }
}

impl Debug for CliFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliFlag")
            .field("flag_char", &self.flag_char)
            .field("name", &self.long_flag)
            .field(
                "about",
                &format!("{}\n{}", self.short_about, self.long_about),
            )
            .field("storing", &self.storing)
            .field("required_store", &self.required_store)
            .finish()
    }
}
