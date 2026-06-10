use std::os::fd::AsRawFd;

use crate::{Cli, CliCommand, CliFlag};

const BREAK: &str = "\n";
const SECTION_BREAK: &str = "\n\n";
const SPACE: &str = " ";

/// # Note
///
/// For simplicity, ASCII is assumed
pub fn help(cli: &Cli) -> String {
    // Arbitrary cap, should be big enough for most, small, programs
    let mut out = String::with_capacity(2048);
    let ident = Indentation::new();

    out.push_str(&make_header(cli, &ident));
    out.push_str(&make_body(cli, &ident));
    out.push_str(&make_footer(&ident));
    out
}

fn make_body(cli: &Cli, ident: &Indentation) -> String {
    let mut out = String::with_capacity(1536);
    out.push_str(&cli.about);
    out.push_str(SECTION_BREAK);

    if cli.flags.len() > 0 {
        out.push_str("All available flags:");
        out.push_str(SECTION_BREAK);
        for flag in &cli.flags {
            out.push_str(&make_flag(flag, ident));
        }
    }
    if cli.sub_commands.len() > 0 {
        if cli.flags.len() > 0 {
            out.push_str(SECTION_BREAK);
            out.push_str(SECTION_BREAK);
        }
        out.push_str("All available commands:");
        out.push_str(SECTION_BREAK);
        for command in &cli.sub_commands {
            out.push_str(&make_subcmd(command, ident));
        }
    }

    out
}

fn make_subcmd(cmd: &Box<dyn CliCommand>, ident: &Indentation) -> String {
    todo!()
}

fn make_flag(flag: &CliFlag, ident: &Indentation) -> String {
    todo!()
}

fn make_header(cli: &Cli, ident: &Indentation) -> String {
    let mut out: Vec<&str> = Vec::with_capacity(3);
    let mut name = cli.name.clone();
    let version = format!("Version: {}", cli.version);
    if ident.is_full(name.len().saturating_add(version.len())) {
        name.push_str(BREAK);
    } else {
        name.push_str(SPACE);
    }
    out.push(&name);
    out.push(&version);
    out.push(SECTION_BREAK);
    out.join("")
}

fn make_footer(ident: &Indentation) -> String {
    let mut out = String::with_capacity(256);
    let mut provider = format!(
        "This CLI experience is provided by Eshu, version {}.",
        env!("CARGO_PKG_VERSION")
    );
    let info = format!("For more information, visit {}", env!("CARGO_PKG_HOMEPAGE"));
    if ident.is_full(provider.len().saturating_add(info.len())) {
        provider.push_str(BREAK);
    } else {
        provider.push_str(SPACE);
    }
    out.push_str(&provider);
    out.push_str(&info);
    out
}

/// Helper struct for indentation
struct Indentation {
    /// The maximum width of the terminal (divided by 2)
    pub max_width: u16,
    /// The amount of indentation per level
    pub amount: u16,
}

impl Indentation {
    /// Create a new indentation instance
    pub fn new() -> Indentation {
        let (_, width) = athena::system::terminal_size(std::io::stdout().as_raw_fd()).unwrap();
        let max_width = width.saturating_div(2);
        let amount = max_width.saturating_div(3);
        Indentation { max_width, amount }
    }
    /// Calculate the indentation for a given level
    pub fn for_level(&self, level: u16) -> u16 {
        self.amount.saturating_mul(level)
    }
    /// Check if the indentation is full
    pub fn is_full(&self, length: usize) -> bool {
        length >= self.max_width as usize
    }
    /// Calculate the padding need to be added for a given level
    pub fn make_padding(&self, cur_len: usize, level: u16) -> String {
        let mut out = String::with_capacity(256);
        let amount = self.for_level(level).saturating_sub(cur_len as u16);
        for _ in 0..amount {
            out.push_str(SPACE);
        }
        out
    }
}
