use crate::{
    CliFlag, EshuError, StoreSyntax, StoreType,
    error::EshuResult,
    utils::{contains_whitespace, starts_with_dash},
};

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
    /// Change the about text
    ///
    /// # Parameters
    ///
    /// * `short_about` - The short about text
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
    ///     .with_about("short about", "long about")
    ///     .build(); // build ensures the entire flag built is valid (about text is all provided here)
    /// assert!(flag.is_ok());
    /// ```
    pub fn with_about(mut self, short_about: &str, long_about: &str) -> CliFlagBuilder {
        self.short_about = short_about.to_string();
        self.long_about = long_about.to_string();
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
    /// # Returns
    ///
    /// * `CliFlagBuilder`
    ///
    /// # Example
    /// ```
    /// use eshu::{CliFlag, StoreType, StoreSyntax};
    ///
    /// let flag = CliFlag::new("flag-name")
    ///     .with_required_store(StoreType::Value, StoreSyntax::Attached)
    ///     .build(); // build ensures the entire flag built is valid (missing about text here)
    /// assert!(flag.is_err());
    /// ```
    pub fn with_required_store(
        mut self,
        store_type: StoreType,
        store_syntax: StoreSyntax,
    ) -> CliFlagBuilder {
        self.storing = true;
        self.required_store = true;
        self.store_type = Some(store_type);
        self.store_syntax = Some(store_syntax);
        self
    }
    /// Mark the flag as storing, meaning it accepts arguments (but does not require them)
    /// If a flag is built by calling `with_required_store`, calling this function is superfluous.
    /// Use this function if the flag accepts arguments optionally
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
