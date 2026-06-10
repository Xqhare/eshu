use std::collections::BTreeMap;

use athena::{Array, Object, XffValue};

/// Check if a string starts with a dash `-`
pub fn starts_with_dash(s: &str) -> bool {
    s.starts_with('-')
}

/// Check if a string contains whitespace
pub fn contains_whitespace(s: &str) -> bool {
    for char in s.chars() {
        if char.is_whitespace() {
            return true;
        }
    }
    false
}

/// Check if an argument is a positional argument
///
/// A positional argument is an argument that does not start with a dash
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
/// Implements `Into<XffValue>`.
///
/// * `Exists` - If the flag does not require a store, this is returned (This is used to indicate
/// that a flag was passed in or exists in the argument stream).
/// * `Value` - A `Vec<String>` (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed values.)
/// * `KeyValue` - A `BTreeMap<String, String>` (All flags can be passed multiple times. It is up to the implementation how exactly to handle multiple passed key-value pairs.)
#[derive(Debug)]
pub enum Store {
    /// The flag was passed in
    ///
    /// Can be cast to `XffValue::Boolean(true)` via `Into<XffValue>`
    Exists,
    /// These values were passed in
    ///
    /// Can be cast to `XffValue::Array` via `Into<XffValue>`
    Value(Vec<String>),
    /// These key-value pairs were passed in
    ///
    /// Can be cast to `XffValue::Object` via `Into<XffValue>`
    KeyValue(BTreeMap<String, String>),
}

impl Into<XffValue> for Store {
    fn into(self) -> XffValue {
        match self {
            Store::Exists => XffValue::from(true),
            Store::Value(val) => {
                let mut out = Array::new();
                for val in val {
                    out.push(XffValue::from(val));
                }
                XffValue::from(out)
            }
            Store::KeyValue(val) => {
                let mut out = Object::new();
                for (key, val) in val {
                    out.insert(key, XffValue::from(val));
                }
                XffValue::from(out)
            }
        }
    }
}

impl Store {
    /// Check if the store exists
    ///
    /// If true, the flag was passed in and no value was provided
    pub fn exists(&self) -> bool {
        match self {
            Store::Exists => true,
            _ => false,
        }
    }
    /// Check if the store is storing a value
    pub fn is_value(&self) -> bool {
        match self {
            Store::Value(_) => true,
            _ => false,
        }
    }
    /// Check if the store is storing a key-value pair
    pub fn is_key_value(&self) -> bool {
        match self {
            Store::KeyValue(_) => true,
            _ => false,
        }
    }
    /// Get the stored value
    pub fn as_value(&self) -> Option<&Vec<String>> {
        match self {
            Store::Value(val) => Some(val),
            _ => None,
        }
    }
    /// Get the stored value as a mutable reference
    pub fn as_mut_value(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Store::Value(val) => Some(val),
            _ => None,
        }
    }
    /// Get the stored key-value pair
    pub fn as_key_value(&self) -> Option<&BTreeMap<String, String>> {
        match self {
            Store::KeyValue(val) => Some(val),
            _ => None,
        }
    }
    /// Get the stored key-value pair as a mutable reference
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
