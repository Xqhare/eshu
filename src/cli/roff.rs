use crate::{Cli, CliCommand, CliFlag, utils::RoffString};

/// Generate a roff manpage
///
/// Only the most basic information is included
pub fn generate_roff_manpage(cli: &Cli) -> RoffString {
    let mut out = RoffString::with_capacity(2048); // Arbitrary cap, should be big enough for most, small, programs (remember it's a string)

    make_th(cli, &mut out);
    make_sh_name(cli, &mut out);
    make_sh_description(cli, &mut out);
    make_flags(cli, &mut out);
    make_commands(cli, &mut out);
    make_author(cli, &mut out);

    out
}

const SPACE: &str = " ";
const BREAK: &str = "\n";

fn make_th(cli: &Cli, out: &mut RoffString) {
    out.push_str(".TH");
    out.push_str(SPACE);
    out.push_str(&cli.name);
    out.push_str(SPACE);
    out.push('1'); // User commands are always version 1 (like `$ man git`)
    out.push_str(SPACE);
    out.push('"');
    out.push_str(&cli.publish_date);
    out.push('"');
    out.push_str(SPACE);
    out.push('"');
    out.push_str(&cli.version);
    out.push('"');
    out.push_str(SPACE);
    out.push('"');
    out.push_str("User Commands");
    out.push('"');
    out.push_str(BREAK);
}

fn make_sh_name(cli: &Cli, out: &mut RoffString) {
    out.push_str(".SH");
    out.push_str(SPACE);
    out.push_str("NAME");
    out.push_str(BREAK);
    out.push_str(&cli.name);
    out.push_str(BREAK);
}

fn make_sh_description(cli: &Cli, out: &mut RoffString) {
    out.push_str(".SH");
    out.push_str(SPACE);
    out.push_str("DESCRIPTION");
    out.push_str(BREAK);
    out.push_str(&cli.about);
    out.push_str(BREAK);
}

fn make_flags(cli: &Cli, out: &mut RoffString) {
    if !cli.flags.is_empty() {
        out.push_str(".PP");
        out.push_str(BREAK);
        out.push_str(".SH");
        out.push_str(SPACE);
        out.push_str("FLAGS");
        out.push_str(BREAK);
        out.push_str(".TP");
        out.push_str(BREAK);

        for flag in &cli.flags {
            out.push_str(".BR");
            make_flag(flag, out);
            out.push_str(".TP");
            out.push_str(BREAK);
        }
    }
}

fn make_flag(flag: &CliFlag, out: &mut RoffString) {
    out.push_str(SPACE);
    if let Some(char) = flag.flag_char {
        out.push('-');
        out.push(char);
        out.push_str(SPACE);
    } else {
        out.push_str(SPACE);
        out.push_str(SPACE);
        out.push_str(SPACE);
    }
    out.push_str("--");
    out.push_str(&flag.long_flag);
    out.push_str(BREAK);
    out.push_str(&flag.short_about);
    out.push_str(SPACE);
    out.push_str(&flag.long_about);
    out.push_str(BREAK);
}

fn make_commands(cli: &Cli, out: &mut RoffString) {
    if !cli.sub_commands.is_empty() {
        out.push_str(".PP");
        out.push_str(BREAK);
        out.push_str(".SH");
        out.push_str(SPACE);
        out.push_str("COMMANDS");
        out.push_str(BREAK);
        out.push_str(".TP");
        out.push_str(BREAK);

        for command in &cli.sub_commands {
            out.push_str(".BR");
            make_subcmd(command.as_ref(), out);
            out.push_str(".TP");
            out.push_str(BREAK);
        }
    }
}

fn make_subcmd(cmd: &dyn CliCommand, out: &mut RoffString) {
    out.push_str(SPACE);
    out.push_str(&cmd.name());
    out.push_str(BREAK);
    out.push_str(&cmd.short_about());
    out.push_str(SPACE);
    out.push_str(&cmd.long_about());
    out.push_str(BREAK);
    if !cmd.flags().is_empty() {
        out.push_str(".PP");
        out.push_str(BREAK);
        out.push_str(".SH");
        out.push_str(SPACE);
        out.push_str("FLAGS");
        out.push_str(BREAK);
        out.push_str(".TP");
        out.push_str(BREAK);
        for flag in cmd.flags() {
            make_flag(flag, out);
        }
    }
    if !cmd.subcommands().is_empty() {
        out.push_str(".PP");
        out.push_str(BREAK);
        out.push_str(".SH");
        out.push_str(SPACE);
        out.push_str("COMMANDS");
        out.push_str(BREAK);
        out.push_str(".TP");
        out.push_str(BREAK);
        for command in cmd.subcommands() {
            make_subcmd(command.as_ref(), out);
        }
    }
}

fn make_author(cli: &Cli, out: &mut RoffString) {
    out.push_str(".SH");
    out.push_str(SPACE);
    out.push_str("AUTHOR");
    out.push_str(BREAK);
    out.push_str(&cli.author);
    out.push_str(BREAK);
}
