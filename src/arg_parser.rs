use std::collections::BTreeMap;

use crate::{
    Cli,
    cli::CliBuilder,
    error::EshuResult,
    utils::{Store, get_params_make_args, starts_with_dash},
};

pub fn parse_args(cli_builder: CliBuilder) -> EshuResult<Cli> {
    cli_builder.validate_self()?;

    let mut entered_flags: BTreeMap<String, (usize, Store)> = BTreeMap::new();
    let mut unknown_args: Vec<String> = Vec::new();
    let params = get_params_make_args();
    let mut args = params.iter().peekable();

    while let Some(arg) = args.next() {
        if unknown_args.len() > 0 && !cli_builder.handle_unknown_args {
            break;
        }

        let mut state = State::Positional;

        if starts_with_dash(arg) {
            if arg.len() == 2 {
                state = State::ShortFlag;
            } else if arg.len() > 2 {
                if arg.starts_with("--") {
                    state = State::LongFlag;
                } else {
                    state = State::Group;
                }
            }
        }

        match state {
            State::ShortFlag => match parse_short_flag(arg, &cli_builder) {
                Some((long_flag, (index, store))) => {
                    entered_flags.insert(long_flag, (index, store));
                }
                None => unknown_args.push(arg.to_string()),
            },
            State::LongFlag => {}
            State::Group => {}
            State::Positional => {
                if is_subcommand(arg, &cli_builder) {
                    todo!("Parse subcommand (dont forget partial matching if unique)");
                }
                unknown_args.push(arg.to_string())
            }
        }
    }

    let unknown_args: Option<Vec<String>> = if cli_builder.handle_unknown_args {
        Some(unknown_args)
    } else {
        todo!("Print error message to user & exit");
    };

    Ok(Cli {
        name: cli_builder.name,
        version: cli_builder.version.unwrap(),
        about: cli_builder.about,
        flags: cli_builder.flags,
        sub_commands: cli_builder.sub_commands,
        entered_flags,
        unknown_args,
    })
}

fn is_subcommand(arg: &str, cli: &CliBuilder) -> bool {
    for subcommand in cli.sub_commands.iter() {
        if subcommand.name() == arg {
            return true;
        }
    }
    false
}

/// Parse a short flag
///
/// Expects `-f` exclusively (including the dash, length of 2), but does not check
fn parse_short_flag(arg: &str, cli: &CliBuilder) -> Option<(String, (usize, Store))> {
    for (index, flag) in cli.flags.iter().enumerate() {
        if flag.flag_char == Some(arg.chars().last().unwrap()) {
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
