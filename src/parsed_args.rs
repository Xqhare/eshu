use std::{collections::BTreeMap, env::args_os};

/// Parse command line arguments
#[derive(Debug, Clone)]
pub struct ParsedArgs {
    flags_by_char: BTreeMap<char, bool>,
    flags_by_name: BTreeMap<String, bool>,
    options_by_char: BTreeMap<char, String>,
    options_by_name: BTreeMap<String, String>,
    kv_by_char: BTreeMap<char, (String, String)>,
    kv_by_name: BTreeMap<String, (String, String)>,
    positionals: Vec<String>,
}

impl ParsedArgs {
    pub fn new() -> Self {
        let mut flags_by_char = BTreeMap::new();
        let mut flags_by_name = BTreeMap::new();
        let mut options_by_char = BTreeMap::new();
        let mut options_by_name = BTreeMap::new();
        let mut kv_by_char = BTreeMap::new();
        let mut kv_by_name = BTreeMap::new();
        let mut positionals = Vec::new();

        let mut stopper_found = false;
        let mut args = args_os().peekable();

        while let Some(arg) = args.next() {
            let arg = arg.to_string_lossy().to_string();
            if arg == "--" {
                stopper_found = true;
                continue;
            }
            if stopper_found {
                positionals.push(arg);
                continue;
            }
            if is_positional(arg.as_str()) {
                positionals.push(arg);
            } else {
                if let Some(next_arg) = args.peek() {
                    let next_arg = next_arg.to_string_lossy().to_string();
                    if is_positional(next_arg.as_str()) {
                        if arg.len() == 2 {
                            if next_arg.contains('=') {
                                let (key, value) = next_arg.split_once('=').unwrap();
                                let char = arg.replace('-', "");
                                debug_assert!(char.len() == 1);
                                kv_by_char.insert(
                                    char.chars().next().unwrap(),
                                    (key.to_string(), value.to_string()),
                                );
                            } else {
                                let char = arg.replace('-', "");
                                debug_assert!(char.len() == 1);
                                options_by_char.insert(char.chars().next().unwrap(), next_arg);
                            }
                        } else {
                        }
                    } else {
                    }
                }
            }
        }
        ParsedArgs {
            flags_by_char,
            flags_by_name,
            options_by_char,
            options_by_name,
            kv_by_char,
            kv_by_name,
            positionals,
        }
    }
    pub fn flag(&self, query: &str) -> bool {
        if query.len() == 1 {
            *self
                .flags_by_char
                .get(&query.chars().next().unwrap())
                .unwrap_or(&false)
        } else {
            *self.flags_by_name.get(query).unwrap_or(&false)
        }
    }
    pub fn option(&self, query: &str) -> Option<String> {
        if query.len() == 1 {
            self.options_by_char
                .get(&query.chars().next().unwrap())
                .cloned()
        } else {
            self.options_by_name.get(query).cloned()
        }
    }
    pub fn kv(&self, query: &str) -> Option<(String, String)> {
        if query.len() == 1 {
            self.kv_by_char.get(&query.chars().next().unwrap()).cloned()
        } else {
            self.kv_by_name.get(query).cloned()
        }
    }
    pub fn args(&self) -> Vec<String> {
        self.positionals.clone()
    }
}

fn is_positional(arg: &str) -> bool {
    if arg.starts_with('-') { false } else { true }
}
