use std::fmt::Debug;

use crate::{
    EshuError, StoreSyntax, StoreType,
    control::builder::CliFlagBuilder,
    error::EshuResult,
    utils::{contains_whitespace, starts_with_dash},
};
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
