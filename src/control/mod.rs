mod builder;
mod cmd;
mod flag;

pub use cmd::CliCmd;
pub use cmd::CliCommand;
pub use flag::CliFlag;

/// The type of the store
///
/// * `Value` - A value (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed values.)
/// * `KeyValue` - A key-value pair (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed key-value pairs.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSyntax {
    /// Attached to the flag; `--flag-name=value` or `--flag-name=key=value`
    Attached,
    /// Detached from the flag; `--flag-name value` or `--flag-name "a value with spaces"` or `--flag-name key=value`
    Detached,
}
