mod builder;
mod cmd;
mod flag;

pub use cmd::CliCmd;
pub use cmd::CliCommand;
pub use flag::CliFlag;

/// The type of data stored by a parsed flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    /// Stores one or more standalone values (e.g. `--file input.txt`).
    Value,
    /// Stores one or more key-value pairs (e.g. `--define key=value`).
    KeyValue,
}

/// The syntax used to pass value arguments to a flag.
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
