use std::collections::BTreeMap;

use crate::{
    Cli, CliFlag, StoreSyntax, StoreType,
    cli::CliBuilder,
    error::EshuResult,
    utils::{Store, is_positional, starts_with_dash, write_err_and_exit},
};

pub fn parse_args(cli_builder: CliBuilder, params: Vec<String>) -> EshuResult<Cli> {
    cli_builder.validate_self()?;

    let mut entered_flags: BTreeMap<String, (usize, Store)> = BTreeMap::new();
    let mut unknown_args: Vec<String> = Vec::new();
    let mut args = params.iter().peekable();
    let mut params_index = 0;

    while let Some(arg) = args.next() {
        params_index += 1;
        if unknown_args.len() > 0 && !cli_builder.handle_unknown_args {
            break;
        }

        let mut state = State::Positional;

        if starts_with_dash(arg) {
            if arg.len() == 2 {
                state = State::ShortFlag;
                if arg == "--" {
                    while let Some(arg) = args.next() {
                        unknown_args.push(arg.to_string());
                    }
                    break;
                }
            } else if arg.len() > 2 {
                if arg.starts_with("--") {
                    state = State::LongFlag;
                } else {
                    state = State::Group;
                }
            }
        }

        let mut next_arg = args.peek().map(|s| s.as_str());
        let mut tmp_next_arg = "".to_string();
        if next_arg == Some("--") {
            // Must be detached Value; For now just combine all following and dump on user
            while let Some(arg) = args.next() {
                tmp_next_arg.push_str(" ");
                tmp_next_arg.push_str(&arg);
            }
            next_arg = Some(&tmp_next_arg);
        }
        match state {
            State::ShortFlag => {
                match parse_short_flag(arg, &cli_builder, next_arg) {
                    Some((long_flag, (index, store))) => {
                        // Already set flag, persist any values also set
                        if !entered_flags.contains_key(&long_flag) {
                            entered_flags.insert(long_flag, (index, store));
                        }
                    }
                    None => unknown_args.push(arg.to_string()),
                }
            }
            State::LongFlag => match parse_long_flag(arg, &cli_builder, next_arg) {
                Some((long_flag, (index, store))) => {
                    insert_long_flag(&mut entered_flags, long_flag, index, store, &cli_builder)
                }
                None => unknown_args.push(arg.to_string()),
            },
            State::Group => {
                let grouped_flags = parse_grouped_flags(arg, &cli_builder, next_arg);
                for (long_flag, (index, store)) in grouped_flags {
                    insert_long_flag(&mut entered_flags, long_flag, index, store, &cli_builder)
                }
            }
            State::Positional => {
                // TODO: Seems suboptimal. Should be able to parse subcommands & sub-flags here; Recursion??
                if parse_subcommand(arg, &cli_builder, &params[params_index..].to_vec()) {
                    break;
                } else {
                    unknown_args.push(arg.to_string())
                }
            }
        }
    }

    let unknown_args: Option<Vec<String>> = if cli_builder.handle_unknown_args {
        Some(unknown_args)
    } else {
        write_err_and_exit(&format!(
            "Usage error: Unknown argument(s): {}",
            unknown_args.join("; ")
        ));
        // Program will exit, but compiler doesn't know
        None
    };

    let cli = Cli {
        name: cli_builder.name,
        version: cli_builder.version.unwrap(),
        about: cli_builder.about,
        flags: cli_builder.flags,
        sub_commands: cli_builder.sub_commands,
        entered_flags,
        unknown_args,
    };

    if cli.is_flag_entered("help") {
        cli.print_help();
        std::process::exit(0);
    }
    if cli.is_flag_entered("version") {
        cli.print_version();
        std::process::exit(0);
    }

    Ok(cli)
}

fn parse_grouped_flags(
    arg: &str,
    cli_builder: &CliBuilder,
    next_arg: Option<&str>,
) -> Vec<(String, (usize, Store))> {
    let mut value = None;
    let mut storing: Vec<char> = Vec::new();
    for (i, c) in arg.char_indices() {
        if i == 0 && c == '-' {
            continue;
        }
        if c == '=' {
            value = Some(arg[i + 1..].to_string());
            break;
        }
        if c.is_ascii_alphabetic() {
            storing.push(c);
        } else {
            write_err_and_exit(&format!("Usage error: Flag must be a-z/A-Z. Got: {}", arg));
        }
    }

    let mut out: Vec<(String, (usize, Store))> = Vec::new();

    for (index, c) in storing.iter().enumerate() {
        for (i, flag) in cli_builder.flags.iter().enumerate() {
            if flag.flag_char == Some(*c) {
                if index == storing.len() - 1 {
                    if flag.storing && value.is_none() {
                        if let Some(next_arg) = next_arg {
                            if is_positional(next_arg) {
                                value = Some(next_arg.to_string());
                            } else if flag.required_store {
                                write_err_and_exit(&format!(
                                    "Usage error: Flag {} requires an argument. Not attached value found, detached value found '{}' is not a positional argument.\n\nPlease provide one via the following syntax: '-{} VALUE' or '-{}=VALUE' ",
                                    flag.long_flag, next_arg, *c, *c
                                ));
                            }
                        }
                    }
                    if let Some(value) = value.clone() {
                        if flag.store_type.is_none() {
                            write_err_and_exit(&format!(
                                "Usage error: Flag {} does not take a value. Eshu found the following value passed to it: {}",
                                flag.long_flag, value
                            ));
                        }
                        match flag.store_type.unwrap() {
                            StoreType::Value => {
                                out.push((flag.long_flag.clone(), (i, Store::Value(vec![value]))))
                            }
                            StoreType::KeyValue => {
                                let split = value.split_once('=');
                                if split.is_none() {
                                    write_err_and_exit(&format!(
                                        "Usage error: Expected key=value, got: {}",
                                        value
                                    ));
                                }
                                let (k, v) = split.unwrap();
                                out.push((
                                    flag.long_flag.clone(),
                                    (
                                        i,
                                        Store::KeyValue(BTreeMap::from([(
                                            k.to_string(),
                                            v.to_string(),
                                        )])),
                                    ),
                                ))
                            }
                        }
                    } else {
                        out.push((flag.long_flag.clone(), (i, Store::Exists)));
                    }
                } else {
                    out.push((flag.long_flag.clone(), (i, Store::Exists)));
                }
            }
        }
    }

    out
}

fn insert_long_flag(
    entered_flags: &mut BTreeMap<String, (usize, Store)>,
    long_flag: String,
    index: usize,
    store: Store,
    cli_builder: &CliBuilder,
) {
    if entered_flags.contains_key(&long_flag) {
        let flag_store = entered_flags.get_mut(&long_flag).expect("Key exists");
        match &cli_builder.flags[index]
            .store_type
            .expect("Store type not set")
        {
            StoreType::Value => {
                let inner_store = flag_store.1.as_mut_value().expect("Must be value");
                let to_store = store.as_value().expect("Must be value");
                for val in to_store {
                    inner_store.push(val.to_string());
                }
            }
            StoreType::KeyValue => {
                let inner_store = flag_store.1.as_mut_key_value().expect("Must be key value");
                let to_store = store.as_key_value().expect("Must be key value");
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
fn parse_long_flag(
    arg: &str,
    cli: &CliBuilder,
    next_arg: Option<&str>,
) -> Option<(String, (usize, Store))> {
    let mut arg = arg;
    if let Some(new_arg) = arg.strip_prefix("--") {
        arg = new_arg;
    }
    let mut partials: Vec<(&str, usize)> = Vec::new();

    for (index, flag) in cli.flags.iter().enumerate() {
        if flag.long_flag.starts_with(arg) {
            if flag.long_flag.len() > arg.len() {
                partials.push((&flag.long_flag, index));
            } else {
                if flag.storing {
                    return Some(handle_store(arg, index, flag, next_arg));
                } else {
                    return Some((flag.long_flag.clone(), (index, Store::Exists)));
                }
            }
        }
    }

    if partials.len() == 1 {
        let partial = &cli.flags[partials[0].1];
        if partial.storing {
            return Some(handle_store(arg, partials[0].1, &partial, next_arg));
        } else {
            return Some((partial.long_flag.clone(), (partials[0].1, Store::Exists)));
        }
    }

    None
}

fn handle_store(
    arg: &str,
    index: usize,
    cli_flag: &CliFlag,
    next_arg: Option<&str>,
) -> (String, (usize, Store)) {
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
            StoreSyntax::Attached => &format!("--{}=VALUE", cli_flag.long_flag),
            StoreSyntax::Detached => &format!("--{} VALUE", cli_flag.long_flag),
        };
        write_err_and_exit(&format!(
            "Usage error: Flag '--{}' requires an argument. Please provide one via the following syntax: '{}'",
            cli_flag.long_flag, req_syntax
        ));
    }

    if value.is_some() {
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

    (cli_flag.long_flag.clone(), (index, store))
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
fn parse_subcommand(arg: &str, cli: &CliBuilder, args: &Vec<String>) -> bool {
    let mut partials = Vec::new();
    for subcommand in cli.sub_commands.iter() {
        if subcommand.name().starts_with(arg) {
            if subcommand.name().len() > arg.len() {
                partials.push(subcommand);
            } else {
                subcommand.execute(args);
                return true;
            }
        }
    }
    if partials.len() == 1 {
        partials[0].execute(args);
        return true;
    }
    false
}

/// Parse a short flag
///
/// Expects `-f` exclusively (including the dash, length of 2), but does not check
///
/// # Note
/// Because of the state machine, this function does not handle `-C=value`; It handles detached
/// values however.
fn parse_short_flag(
    arg: &str,
    cli: &CliBuilder,
    next_arg: Option<&str>,
) -> Option<(String, (usize, Store))> {
    for (index, flag) in cli.flags.iter().enumerate() {
        if flag.flag_char == Some(arg.chars().last().unwrap()) {
            if flag.storing {
                if flag.required_store && next_arg.is_none() {
                    write_err_and_exit(&format!(
                        "Usage error: Flag '-{}' (--{}) requires an argument. Please provide one via the following syntax: '-{} VALUE' or '-{}=VALUE'",
                        arg, flag.long_flag, arg, arg
                    ));
                }
                if let Some(next_arg) = next_arg {
                    match flag.store_type.expect("Store type not set") {
                        StoreType::Value => {
                            return Some((
                                flag.long_flag.clone(),
                                (index, Store::Value(vec![next_arg.to_string()])),
                            ));
                        }
                        StoreType::KeyValue => {
                            let (key, val) = next_arg.split_once('=').expect("Must be key=value");
                            return Some((
                                flag.long_flag.clone(),
                                (
                                    index,
                                    Store::KeyValue(BTreeMap::from([(
                                        key.to_string(),
                                        val.to_string(),
                                    )])),
                                ),
                            ));
                        }
                    }
                }
            }
            return Some((flag.long_flag.clone(), (index, Store::Exists)));
        }
    }
    None
}

/// Internal Parser State Machine State
enum State {
    /// Short flag
    ShortFlag,
    /// Long flag
    LongFlag,
    /// Group of short flags
    Group,
    /// Positional argument
    Positional,
}
