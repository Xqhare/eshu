use std::collections::BTreeMap;
use nemesis::{NemesisError, NemesisResultExt};

use crate::{
    Cli, CliFlag, EshuErrorKind, StoreSyntax, StoreType,
    cli::builder::CliBuilder,
    utils::{Store, is_positional},
};

/// Inserts a flag into the entered flags map
pub fn insert_long_flag(
    entered_flags: &mut BTreeMap<String, (usize, Store)>,
    long_flag: String,
    index: usize,
    store: Store,
) {
    if entered_flags.contains_key(&long_flag) {
        let flag_store = entered_flags.get_mut(&long_flag).expect("Key exists");
        match &flag_store.1 {
            Store::Exists => {
                // Do nothing, flag already exists
            }
            Store::Value(_) => {
                let inner_store = flag_store.1.as_mut_value().expect("Must be value");
                let to_store = {
                    match store {
                        Store::Exists => Vec::new(),
                        Store::Value(val) => val,
                        _ => unreachable!("Must be value"),
                    }
                };
                for val in to_store {
                    inner_store.push(val.to_string());
                }
            }
            Store::KeyValue(_) => {
                let inner_store = flag_store.1.as_mut_key_value().expect("Must be key value");
                let to_store = {
                    match store {
                        Store::Exists => BTreeMap::new(),
                        Store::KeyValue(val) => val,
                        _ => unreachable!("Must be key value"),
                    }
                };
                for (key, val) in to_store {
                    inner_store.insert(key.to_string(), val.to_string());
                }
            }
        }
    } else {
        entered_flags.insert(long_flag, (index, store));
    }
}

/// Returns (long_flag, (index, store))
/// Handles both flags with leading `--` and without
///
/// # Note
/// Because of the state machine, this function also has to handle `-C=value`
pub fn parse_long_flag(
    arg: &str,
    cli: &CliBuilder,
    next_arg: Option<&str>,
    detached_list_args: Option<&[String]>,
) -> Result<Option<(String, (usize, Store))>, NemesisError> {
    let mut arg = arg;
    if let Some(new_arg) = arg.strip_prefix("--") {
        arg = new_arg;
    }
    let mut partials: Vec<(&str, usize)> = Vec::new();

    for (index, flag) in cli.flags.iter().enumerate() {
        let parsed_long_flag = {
            if arg.contains('=') {
                let (left, _) = arg.split_once('=').expect("arg contains =");
                left
            } else {
                arg
            }
        };
        if parsed_long_flag == flag.long_flag {
            if flag.storing {
                return Ok(Some(handle_store(arg, index, flag, next_arg, detached_list_args)?));
            } else {
                return Ok(Some((flag.long_flag.clone(), (index, Store::Exists))));
            }
        } else if flag.long_flag.starts_with(parsed_long_flag) {
            partials.push((flag.long_flag.as_str(), index));
        }
    }

    if partials.len() == 1 {
        let partial = &cli.flags[partials[0].1];
        if partial.storing {
            return Ok(Some(handle_store(
                arg,
                partials[0].1,
                &partial,
                next_arg,
                detached_list_args,
            )?));
        } else {
            return Ok(Some((partial.long_flag.clone(), (partials[0].1, Store::Exists))));
        }
    }

    Ok(None)
}

fn handle_store(
    arg: &str,
    index: usize,
    cli_flag: &CliFlag,
    next_arg: Option<&str>,
    detached_list_args: Option<&[String]>,
) -> Result<(String, (usize, Store)), NemesisError> {
    let mut store = Store::Exists;
    let mut value = None;

    match &cli_flag.store_syntax.expect("Store syntax not set") {
        StoreSyntax::Attached => {
            if let Some((_, val)) = arg.split_once('=') {
                value = Some(val);
            }
        }
        StoreSyntax::Detached => {
            if let Some(next_argument) = next_arg
                && is_positional(next_argument)
            {
                value = Some(next_argument);
            }
        }
    }

    if cli_flag.required_store && value.is_none() {
        let req_syntax = match &cli_flag.store_syntax.expect("Store syntax not set") {
            StoreSyntax::Attached => format!("--{}=VALUE", cli_flag.long_flag),
            StoreSyntax::Detached => format!("--{} VALUE", cli_flag.long_flag),
        };
        return Err(NemesisError::new(
            "eshu::parser",
            EshuErrorKind::MissingArgument {
                flag: format!("--{}", cli_flag.long_flag),
                expected_syntax: req_syntax,
            },
        ));
    }

    if let Some(detached_list_args) = detached_list_args {
        match &cli_flag.store_type.expect("Store type not set") {
            StoreType::Value => {
                store = Store::Value(detached_list_args.to_vec());
            }
            StoreType::KeyValue => {
                let mut map = BTreeMap::new();
                for arg in detached_list_args {
                    let (key, val) = arg.split_once('=').expect("Must be key=value");
                    map.insert(key.to_string(), val.to_string());
                }
                store = Store::KeyValue(map);
            }
        }
    } else if value.is_some() {
        match &cli_flag.store_type.expect("Store type not set") {
            StoreType::Value => {
                store = Store::Value(vec![value.unwrap().to_string()]);
            }
            StoreType::KeyValue => {
                let (key, val) = value.unwrap().split_once('=').expect("Must be key=value");
                store = Store::KeyValue(BTreeMap::from([(key.to_string(), val.to_string())]));
            }
        }
    }

    Ok((cli_flag.long_flag.clone(), (index, store)))
}

/// Parse a subcommand
/// Also checks for partial matches
///
/// # Arguments
///
/// * `arg` - The argument to parse
/// * `cli` - The cli builder
/// * `args` - All arguments passed to the program to the immediate right of the subcommand
///
/// # Returns
///
/// * `bool` - Whether or not the subcommand was found. True if found
pub fn parse_subcommand<'a>(
    arg: &str,
    cli: &CliBuilder<'a>,
    args: &Vec<String>,
) -> Result<Option<(String, Cli<'a>)>, NemesisError> {
    if arg.is_empty() {
        return Ok(None);
    }
    let mut partials = Vec::new();
    let mut execute = None;
    for subcommand in cli.sub_commands.iter() {
        if subcommand.name().starts_with(arg) {
            if subcommand.name().len() > arg.len() {
                partials.push(subcommand);
            } else {
                execute = Some(subcommand);
                break;
            }
        }
    }
    if partials.len() == 1 && execute.is_none() {
        execute = Some(partials[0]);
    }
    if let Some(execute) = execute {
        let mut inner_cli = Cli::new(execute.name())
            .with_about(&execute.short_about())
            .with_version(
                &cli.version
                    .clone()
                    .unwrap_or("0.0.0".to_string())
                    .to_string(),
            );
        for flag in execute.flags() {
            inner_cli = inner_cli.add_flag(flag.clone())
        }
        for subcommand in execute.subcommands() {
            inner_cli = inner_cli.add_command(subcommand.clone());
        }
        let mut inner_args = Vec::with_capacity(args.len().saturating_add(1));
        inner_args.push("".to_string());
        inner_args.extend_from_slice(args);

        let inner_cli = inner_cli
            .try_parse_custom(inner_args)
            .add_source("eshu::parser")
            .add_ctx(format!("Parsing arguments for subcommand '{}'", execute.name()))?;

        Ok(Some((execute.name().to_string(), inner_cli)))
    } else {
        Ok(None)
    }
}

/// Parse a short flag
///
/// Expects `-f` exclusively (including the dash, length of 2), but does not check
///
/// # Note
/// Because of the state machine, this function does not handle `-C=value`; It handles detached
/// values however.
pub fn parse_short_flag(
    arg: &str,
    cli: &CliBuilder,
    next_arg: Option<&str>,
    detached_list_args: Option<&[String]>,
) -> Result<Option<(String, (usize, Store))>, NemesisError> {
    for (index, flag) in cli.flags.iter().enumerate() {
        if flag.flag_char == Some(arg.chars().last().unwrap()) {
            if flag.storing {
                let mut value = None;
                if flag.store_syntax == Some(StoreSyntax::Detached) {
                    if let Some(next_argument) = next_arg {
                        if is_positional(next_argument) {
                            value = Some(next_argument.to_string());
                        }
                    }
                }

                if flag.required_store && value.is_none() {
                    let req_syntax = match &flag.store_syntax.expect("Store syntax not set") {
                        StoreSyntax::Attached => {
                            format!("-{}={}", arg.chars().last().unwrap(), "VALUE")
                        }
                        StoreSyntax::Detached => {
                            format!("-{} {}", arg.chars().last().unwrap(), "VALUE")
                        }
                    };
                    return Err(NemesisError::new(
                        "eshu::parser",
                        EshuErrorKind::MissingArgument {
                            flag: format!("-{} (--{})", arg.chars().last().unwrap(), flag.long_flag),
                            expected_syntax: req_syntax,
                        },
                    ));
                }

                if let Some(val) = value {
                    match flag.store_type.expect("Store type not set") {
                        StoreType::Value => {
                            if let Some(detached_list_args) = detached_list_args {
                                // Ignore value here, end-of-flag marker was found
                                return Ok(Some((
                                    flag.long_flag.clone(),
                                    (index, Store::Value(detached_list_args.to_vec())),
                                )));
                            }
                            return Ok(Some((
                                flag.long_flag.clone(),
                                (index, Store::Value(vec![val])),
                            )));
                        }
                        StoreType::KeyValue => {
                            if let Some(detached_list_args) = detached_list_args {
                                // Ignore value here, end-of-flag marker was found
                                let mut map = BTreeMap::new();
                                for arg in detached_list_args {
                                    let (key, val) =
                                        arg.split_once('=').expect("Must be key=value");
                                    map.insert(key.to_string(), val.to_string());
                                }
                                return Ok(Some((
                                    flag.long_flag.clone(),
                                    (index, Store::KeyValue(map)),
                                )));
                            }
                            let (key, v) = val.split_once('=').expect("Must be key=value");
                            return Ok(Some((
                                flag.long_flag.clone(),
                                (
                                    index,
                                    Store::KeyValue(BTreeMap::from([(
                                        key.to_string(),
                                        v.to_string(),
                                    )])),
                                ),
                            )));
                        }
                    }
                }
            }
            return Ok(Some((flag.long_flag.clone(), (index, Store::Exists))));
        }
    }
    Ok(None)
}
