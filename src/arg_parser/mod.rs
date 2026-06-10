use std::collections::BTreeMap;

use crate::{
    Cli,
    arg_parser::parser::{
        insert_long_flag, parse_grouped_flags, parse_long_flag, parse_short_flag, parse_subcommand,
    },
    cli::builder::CliBuilder,
    error::EshuResult,
    utils::{Store, starts_with_dash, write_err_and_exit},
};

mod parser;

pub fn parse_args(cli_builder: CliBuilder, params: Vec<String>) -> EshuResult<Cli> {
    cli_builder.validate_self()?;

    let mut entered_flags: BTreeMap<String, (usize, Store)> = BTreeMap::new();
    let mut unknown_args: Vec<String> = Vec::new();
    let mut sub_cmd_cli: BTreeMap<String, Cli> = BTreeMap::new(); // <name, cli>
    let mut stray_positional_args: Vec<String> = Vec::new();
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
            State::ShortFlag => match parse_short_flag(arg, &cli_builder, next_arg) {
                Some((long_flag, (index, store))) => {
                    if let Store::Value(_) | Store::KeyValue(_) = &store {
                        args.next();
                    }
                    insert_long_flag(&mut entered_flags, long_flag, index, store);
                }
                None => unknown_args.push(arg.to_string()),
            },
            State::LongFlag => match parse_long_flag(arg, &cli_builder, next_arg) {
                Some((long_flag, (index, store))) => {
                    if !arg.contains('=') {
                        if let Store::Value(_) | Store::KeyValue(_) = &store {
                            args.next();
                        }
                    }
                    insert_long_flag(&mut entered_flags, long_flag, index, store)
                }
                None => unknown_args.push(arg.to_string()),
            },
            State::Group => {
                let grouped_flags = parse_grouped_flags(arg, &cli_builder, next_arg);
                let mut consumed_next = false;
                if !arg.contains('=') {
                    for (_, (_, store)) in &grouped_flags {
                        if let Store::Value(_) | Store::KeyValue(_) = store {
                            consumed_next = true;
                        }
                    }
                }
                if consumed_next {
                    args.next();
                }
                for (long_flag, (index, store)) in grouped_flags {
                    insert_long_flag(&mut entered_flags, long_flag, index, store)
                }
            }
            State::Positional => {
                if let Some((name, sub_cli)) =
                    parse_subcommand(arg, &cli_builder, &params[params_index..].to_vec())
                {
                    sub_cmd_cli.insert(name, sub_cli);
                } else {
                    stray_positional_args.push(arg.to_string())
                }
            }
        }
    }

    let unknown_args: Option<Vec<String>> = if cli_builder.handle_unknown_args {
        Some(unknown_args)
    } else if unknown_args.len() > 0 {
        write_err_and_exit(&format!(
            "Usage error: Unknown argument(s): {}",
            unknown_args.join("; ")
        ));
        // Program will exit, but compiler doesn't know
        None
    } else {
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
        stray_positional_args,
        sub_cmd_cli,
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
