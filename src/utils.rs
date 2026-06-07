use std::collections::BTreeMap;

pub fn starts_with_dash(s: &str) -> bool {
    s.starts_with('-')
}

pub fn contains_whitespace(s: &str) -> bool {
    for char in s.chars() {
        if char.is_whitespace() {
            return true;
        }
    }
    false
}

pub fn is_positional(arg: &str) -> bool {
    !starts_with_dash(arg)
}

/// Get the arguments passed to the program
///
/// Assumes the program name is the first argument
/// Also accepts all limitations of using `OsString` and lossy conversion to `String`
pub fn get_params_make_args() -> Vec<String> {
    let args: Vec<String> = std::env::args_os()
        .skip(1) // Assume the program name is the first argument, *ALWAYS*;
        .map(|s| s.to_string_lossy().to_string()) // There will be edge cases. Especially
        // cross platform.
        .collect();
    args
}

/// Roff string type alias for Man page returns
pub type RoffString = String;

/// The store of the flag
/// This is returned by the `Cli::parse` function
/// The type of the store is determined by the `CliFlag::store_type`
///
/// * `Exists` - If the flag does not require a store, this is returned (This is used to indicate
/// that a flag was passed in or exists in the argument stream).
/// * `Value` - A `Vec<String>` (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed values.)
/// * `KeyValue` - A `BTreeMap<String, String>` (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed key-value pairs.)
#[derive(Debug)]
pub enum Store {
    /// The flag was passed in
    Exists,
    /// These values were passed in
    Value(Vec<String>),
    /// These key-value pairs were passed in
    KeyValue(BTreeMap<String, String>),
}

impl Store {
    pub fn exists(&self) -> bool {
        match self {
            Store::Exists => true,
            _ => false,
        }
    }
    pub fn is_value(&self) -> bool {
        match self {
            Store::Value(_) => true,
            _ => false,
        }
    }
    pub fn is_key_value(&self) -> bool {
        match self {
            Store::KeyValue(_) => true,
            _ => false,
        }
    }
    pub fn as_value(&self) -> Option<&Vec<String>> {
        match self {
            Store::Value(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_mut_value(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Store::Value(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_key_value(&self) -> Option<&BTreeMap<String, String>> {
        match self {
            Store::KeyValue(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_mut_key_value(&mut self) -> Option<&mut BTreeMap<String, String>> {
        match self {
            Store::KeyValue(val) => Some(val),
            _ => None,
        }
    }
}

/// Write the message into stderr and exit
pub fn write_err_and_exit(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}
