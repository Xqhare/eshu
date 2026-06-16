use crate::{Cli, CliCommand, CliFlag, StoreSyntax, StoreType, utils::RoffString};

/// Generate a roff manpage
///
/// Only the most basic information is included
pub fn generate_roff_manpage(cli: &Cli) -> RoffString {
    let mut out = RoffString::with_capacity(2048);

    make_th(cli, &mut out);
    make_sh_name(cli, &mut out);
    make_sh_description(cli, &mut out);

    // Top-level Flags
    if !cli.flags.is_empty() {
        out.push_str(".SH FLAGS");
        out.push_str(BREAK);
        for flag in &cli.flags {
            make_flag(flag, &mut out);
        }
    }

    // Top-level Commands
    if !cli.sub_commands.is_empty() {
        out.push_str(".SH COMMANDS");
        out.push_str(BREAK);
        for command in &cli.sub_commands {
            make_subcmd(command.as_ref(), &mut out);
        }
    }

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

    // Escape leading dots or quotes in description
    for line in cli.about.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('.') || trimmed.starts_with('\'') {
            out.push_str("\\&");
        }
        out.push_str(line);
        out.push_str(BREAK);
    }
}

fn make_flag(flag: &CliFlag, out: &mut RoffString) {
    out.push_str(".TP");
    out.push_str(BREAK);

    let escaped_long = flag.long_flag.replace('-', "\\-");

    // Format store suffix (e.g. =VALUE or =KEY=VALUE)
    let store_suffix = if flag.storing {
        let val_str = match flag.store_type {
            Some(StoreType::Value) => "VALUE",
            Some(StoreType::KeyValue) => "KEY=VALUE",
            None => "VALUE",
        };
        match (flag.store_syntax, flag.required_store) {
            (Some(StoreSyntax::Attached), true) => format!("=\\fI{val_str}\\fR"),
            (Some(StoreSyntax::Attached), false) => format!("[=\\fI{val_str}\\fR]"),
            (Some(StoreSyntax::Detached), true) => format!(" \\fI{val_str}\\fR"),
            (Some(StoreSyntax::Detached), false) => format!(" [\\fI{val_str}\\fR]"),
            _ => String::new(),
        }
    } else {
        String::new()
    };

    if let Some(c) = flag.flag_char {
        // Format as: \fB\-c\fR, \fB\-\-long\-flag\fR[suffix]
        out.push_str(&format!(
            "\\fB\\-{c}\\fR, \\fB\\-\\-{escaped_long}\\fR{store_suffix}"
        ));
    } else {
        // Format as: \fB\-\-long\-flag\fR[suffix]
        out.push_str(&format!("\\fB\\-\\-{escaped_long}\\fR{store_suffix}"));
    }
    out.push_str(BREAK);

    // Build descriptions
    let mut desc = flag.short_about.clone();
    if !flag.long_about.is_empty() {
        if !desc.is_empty() {
            desc.push(' ');
        }
        desc.push_str(&flag.long_about);
    }

    for line in desc.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('.') || trimmed.starts_with('\'') {
            out.push_str("\\&");
        }
        out.push_str(line);
        out.push_str(BREAK);
    }
}

fn make_subcmd(cmd: &dyn CliCommand, out: &mut RoffString) {
    out.push_str(".TP");
    out.push_str(BREAK);
    out.push_str(&format!("\\fB{}\\fR", cmd.name()));
    out.push_str(BREAK);

    // Subcommand description (short_about + long_about)
    let mut desc = cmd.short_about();
    let long_about = cmd.long_about();
    if !long_about.is_empty() {
        if !desc.is_empty() {
            desc.push(' ');
        }
        desc.push_str(&long_about);
    }

    for line in desc.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('.') || trimmed.starts_with('\'') {
            out.push_str("\\&");
        }
        out.push_str(line);
        out.push_str(BREAK);
    }

    // Check if there are flags or subcommands inside this command
    let has_flags = !cmd.flags().is_empty();
    let subcmds = cmd.subcommands();
    let has_subcmds = !subcmds.is_empty();

    if has_flags || has_subcmds {
        out.push_str(".RS");
        out.push_str(BREAK);

        if has_flags {
            out.push_str(".SS FLAGS");
            out.push_str(BREAK);
            for flag in cmd.flags() {
                make_flag(flag, out);
            }
        }

        if has_subcmds {
            out.push_str(".SS COMMANDS");
            out.push_str(BREAK);
            for subcmd in subcmds {
                make_subcmd(subcmd.as_ref(), out);
            }
        }

        out.push_str(".RE");
        out.push_str(BREAK);
    }
}

fn make_author(cli: &Cli, out: &mut RoffString) {
    if !cli.author.is_empty() {
        out.push_str(".SH AUTHOR");
        out.push_str(BREAK);

        for line in cli.author.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('.') || trimmed.starts_with('\'') {
                out.push_str("\\&");
            }
            out.push_str(line);
            out.push_str(BREAK);
        }
    }
}
