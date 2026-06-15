use std::{collections::BTreeMap, process::exit};

use crate::{
    Cli, EshuErrorKind,
    arg_parser::{
        grouped::parse_grouped_flags,
        parser::{insert_long_flag, parse_long_flag, parse_short_flag, parse_subcommand},
    },
    cli::builder::CliBuilder,
    error::EshuResult,
    utils::{Store, starts_with_dash},
};
use nemesis::NemesisError;

mod grouped;
mod parser;

#[expect(clippy::expect_used, reason = "Dynamic check done in validate_self")]
#[expect(clippy::shadow_unrelated, reason = "Shadowing is fine here")]
#[expect(clippy::needless_pass_by_value, reason = "API")]
#[expect(clippy::too_many_lines, reason = "Parsing is complex")]
#[expect(clippy::cognitive_complexity, reason = "Parsing is complex")]
pub fn parse_args(cli_builder: CliBuilder, params: Vec<String>) -> EshuResult<Cli> {
    cli_builder.validate_self()?;

    let mut entered_flags: BTreeMap<String, (usize, Store)> = BTreeMap::new();
    let mut unknown_args: Vec<String> = Vec::new();
    let mut sub_cmd_cli: BTreeMap<String, Cli> = BTreeMap::new(); // <name, cli>
    let mut stray_positional_args: Vec<String> = Vec::new();
    if !params.is_empty() {
        let mut args = params[1..].iter().peekable();
        let mut params_index: usize = 1;

        while let Some(arg) = args.next() {
            params_index = params_index.saturating_add(1);
            if !unknown_args.is_empty() && !cli_builder.handle_unknown_args {
                break;
            }

            let mut state = State::Positional;

            if starts_with_dash(arg) {
                if arg.len() == 2 {
                    if arg == "--" {
                        for arg in args.by_ref() {
                            stray_positional_args.push(arg.clone());
                        }
                        break;
                    }
                    state = State::ShortFlag;
                } else if arg.len() > 2 {
                    if arg.starts_with("--") {
                        state = State::LongFlag;
                    } else {
                        state = State::Group;
                    }
                }
            }

            let mut next_arg = args.peek().map(|s| s.as_str());
            let mut buf: Option<&[String]> = None;
            if next_arg == Some("--") {
                next_arg = None;
                buf = Some(&params[params_index.saturating_add(1)..]);
            }
            match state {
                State::ShortFlag => match parse_short_flag(arg, &cli_builder, next_arg, buf)? {
                    Some((long_flag, (index, store))) => {
                        if let Store::Value(_) | Store::KeyValue(_) = &store {
                            args.next();
                        } else if let Some(buf) = buf {
                            let buf = &mut buf.to_vec();
                            stray_positional_args.append(buf);
                        }
                        insert_long_flag(&mut entered_flags, long_flag, index, store);
                    }
                    None => unknown_args.push(arg.clone()),
                },
                State::LongFlag => match parse_long_flag(arg, &cli_builder, next_arg, buf)? {
                    Some((long_flag, (index, store))) => {
                        if !arg.contains('=') {
                            if let Store::Value(_) | Store::KeyValue(_) = &store {
                                args.next();
                            } else if let Some(buf) = buf {
                                let buf = &mut buf.to_vec();
                                stray_positional_args.append(buf);
                            }
                        }
                        insert_long_flag(&mut entered_flags, long_flag, index, store);
                    }
                    None => unknown_args.push(arg.clone()),
                },
                State::Group => {
                    let grouped_flags = parse_grouped_flags(arg, &cli_builder, next_arg, buf)?;
                    let mut consumed_next = false;
                    if !arg.contains('=') {
                        for (_, (_, store)) in &grouped_flags {
                            if let Store::Value(_) | Store::KeyValue(_) = store {
                                consumed_next = true;
                            } else if let Some(buf) = buf {
                                let buf = &mut buf.to_vec();
                                stray_positional_args.append(buf);
                                break;
                            }
                        }
                    }
                    if consumed_next {
                        args.next();
                    }
                    for (long_flag, (index, store)) in grouped_flags {
                        insert_long_flag(&mut entered_flags, long_flag, index, store);
                    }
                }
                State::Positional => {
                    if let Some((name, sub_cli)) =
                        parse_subcommand(arg, &cli_builder, &params[params_index..])?
                    {
                        sub_cmd_cli.insert(name, sub_cli);
                        break;
                    } else {
                        stray_positional_args.push(arg.clone());
                    }
                }
            }
            if buf.is_some() {
                break;
            }
        }
    }

    let unknown_args: Option<Vec<String>> = if cli_builder.handle_unknown_args {
        Some(unknown_args)
    } else if !unknown_args.is_empty() {
        return Err(NemesisError::new(
            "eshu::parser",
            EshuErrorKind::UnknownArgument(unknown_args.join("; ")),
        ));
    } else {
        None
    };

    let cli = Cli {
        name: cli_builder.name,
        version: cli_builder.version.expect("Dynamically checked above"),
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
        exit(0);
    }
    if cli.is_flag_entered("version") {
        cli.print_version();
        exit(0);
    }

    if cli_builder.auto_execution {
        for cmd in &cli.sub_cmd_cli {
            for def_cmd in &cli.sub_commands {
                if cmd.0 == &def_cmd.name() {
                    def_cmd.execute(cmd.1);
                }
            }
        }
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
