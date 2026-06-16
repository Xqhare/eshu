use std::fmt::{self, Write as _};
use std::io::stdout;
use std::os::fd::AsRawFd as _;

use athena::system::terminal_size;

use crate::{Cli, CliCommand, CliFlag, StoreSyntax, StoreType};

const BREAK: &str = "\n";
const SECTION_BREAK: &str = "\n\n";
const SPACE: &str = " ";

/// # Note
///
/// For simplicity, ASCII is assumed
pub fn help(cli: &Cli) -> String {
    // Arbitrary cap, should be big enough for most, small, programs (remember it's a string)
    let mut out = TermWriter::new(2048);

    make_header(cli, &mut out);
    make_body(cli, &mut out);
    make_footer(&mut out);

    out.buffer
}

fn make_body(cli: &Cli, out: &mut TermWriter) {
    out.push_str(&cli.about);
    out.push_str(SECTION_BREAK);

    if !cli.flags.is_empty() {
        out.push_str("All available flags:");
        out.push_str(SECTION_BREAK);
        for flag in &cli.flags {
            make_flag(flag, out);
            out.push_str(SECTION_BREAK);
        }
        out.buffer
            .truncate(out.buffer.len().saturating_sub(SECTION_BREAK.len()));
    }
    if !cli.sub_commands.is_empty() {
        if !cli.flags.is_empty() {
            out.push_str(SECTION_BREAK);
            out.push_str(SECTION_BREAK);
        }
        out.push_str("All available commands:");
        out.push_str(SECTION_BREAK);
        for command in &cli.sub_commands {
            make_subcmd(command.as_ref(), out);
            out.push_str(SECTION_BREAK);
        }
        out.buffer
            .truncate(out.buffer.len().saturating_sub(SECTION_BREAK.len()));
    }
    out.push_str(SECTION_BREAK);
}

fn make_subcmd(cmd: &dyn CliCommand, out: &mut TermWriter) {
    out.push_str(SPACE);
    out.push_str(&cmd.name());
    out.pad_to_column(1);
    out.wrap_text(&format!("{}\n{}", cmd.short_about(), cmd.long_about()), 1);
    out.push_str(BREAK);
    let flags = cmd.flags();
    if !flags.is_empty() {
        out.push_str("All available flags for this command:");
        out.push_str(SECTION_BREAK);
        for flag in flags {
            make_flag(flag, out);
            out.push_str(SECTION_BREAK);
        }
        out.buffer
            .truncate(out.buffer.len().saturating_sub(SECTION_BREAK.len()));
    }
    let sub_commands = cmd.subcommands();
    if !sub_commands.is_empty() {
        if !flags.is_empty() {
            out.push_str(SECTION_BREAK);
            out.push_str(SECTION_BREAK);
        }
        out.push_str("All available sub-commands for this command:");
        out.push_str(SECTION_BREAK);
        for command in sub_commands {
            make_subcmd(command.as_ref(), out);
            out.push_str(SECTION_BREAK);
        }
        out.buffer
            .truncate(out.buffer.len().saturating_sub(SECTION_BREAK.len()));
    }
    out.push_str(BREAK);
}

#[expect(
    clippy::let_underscore_must_use,
    reason = "Dont want to make this error"
)]
fn make_flag(flag: &CliFlag, out: &mut TermWriter) {
    out.push_str(SPACE);

    if let Some(short) = &flag.flag_char {
        let _ = write!(out, "-{short}");
    } else {
        out.push_str(SPACE); // Spaces for missing short flag
        out.push_str(SPACE);
    }

    out.pad_to_column(1);
    let _ = write!(out, "--{}", flag.long_flag);

    out.pad_to_column(2);
    out.push_str("Usage: ");
    make_flag_syntax(flag, out);

    out.pad_to_column(3);
    out.wrap_text(&flag.short_about, 3);
    out.push_str(BREAK);

    if !flag.long_flag.is_empty() {
        out.pad_to_column(3);
        out.wrap_text(&flag.long_about, 3);
        out.push_str(BREAK);
    }
}

fn make_flag_syntax(flag: &CliFlag, out: &mut TermWriter) {
    // Format store suffix (e.g. =VALUE or =KEY=VALUE)
    let store_suffix = if flag.storing {
        let val_str = match flag.store_type {
            Some(StoreType::Value) => "VALUE",
            Some(StoreType::KeyValue) => "KEY=VALUE",
            None => "VALUE",
        };
        match (flag.store_syntax, flag.required_store) {
            (Some(StoreSyntax::Attached), true) => format!("={val_str}"),
            (Some(StoreSyntax::Attached), false) => format!("[={val_str}]"),
            (Some(StoreSyntax::Detached), true) => format!(" {val_str}"),
            (Some(StoreSyntax::Detached), false) => format!(" [{val_str}]"),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    if let Some(c) = flag.flag_char {
        out.push_str("-");
        out.push_str(&c.to_string());
        out.push_str(&store_suffix);
        out.push_str(SPACE);
    } else {
        out.push_str(SPACE);
        out.push_str(SPACE);
        for _ in store_suffix.chars() {
            out.push_str(SPACE);
        }
        out.push_str(SPACE);
    }

    out.push_str("--");
    out.push_str(&flag.long_flag);
    out.push_str(&store_suffix);
}

fn make_header(cli: &Cli, out: &mut TermWriter) {
    out.push_str(&cli.name.clone());
    out.push_str(BREAK);
    out.push_str(&format!("Version: {}", cli.version));
    out.push_str(SECTION_BREAK);
}

fn make_footer(out: &mut TermWriter) {
    out.wrap_text(
        &format!(
            "This CLI experience is provided by Eshu, version {}.\nFor more information, visit {}",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_HOMEPAGE")
        ),
        0,
    );
}

/// Helper struct for indentation
struct Indentation {
    /// The maximum width of the terminal (divided by 2)
    pub max_width: usize,
    /// The amount of indentation per level
    pub amount: usize,
}

impl Indentation {
    /// Create a new indentation instance
    pub fn new() -> Indentation {
        let (_, max_width) = terminal_size(stdout().as_raw_fd()).unwrap_or((0, 80));
        let amount = max_width.saturating_div(3).clamp(20, 40) as usize;
        Indentation {
            max_width: max_width as usize,
            amount,
        }
    }
}

/// Helper struct for writing the help buffer
///
/// Takes care of indentation, wrapping, etc
#[expect(clippy::partial_pub_fields, reason = "API")]
pub struct TermWriter {
    /// The buffer, use as final output
    pub buffer: String,
    /// The current column
    current_col: usize,
    /// The indentation helper
    indent: Indentation,
}

impl TermWriter {
    /// Creates a new `TermWriter`
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            current_col: 0,
            indent: Indentation::new(),
        }
    }

    /// Pushes text and automatically updates the column counter
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);

        if let Some(last_line) = s.rsplit('\n').next() {
            if s.contains('\n') {
                self.current_col = last_line.chars().count();
            } else {
                self.current_col = self.current_col.saturating_add(last_line.chars().count());
            }
        }
    }

    /// Pads the current line with spaces until it hits the target column
    pub fn pad_to_column(&mut self, target_level: usize) {
        let target = self.indent.amount.saturating_mul(target_level);
        let padding_needed = target.saturating_sub(self.current_col);
        for _ in 0..padding_needed {
            self.buffer.push(' ');
        }
        self.current_col = self.current_col.saturating_add(padding_needed);
    }

    /// Zero-dependency text wrapping
    #[expect(
        unused_assignments,
        reason = "`line_start = true` should be needed for the next word loop iteration"
    )]
    #[expect(clippy::else_if_without_else, reason = "Its fine, dont do anything")]
    pub fn wrap_text(&mut self, text: &str, target_level: usize) {
        for line in text.lines() {
            let mut line_start = true;
            for word in line.split_whitespace() {
                let word_len = word.chars().count();

                // If adding this word exceeds the terminal width, wrap to next line
                if self.current_col.saturating_add(word_len).saturating_add(1)
                    > self.indent.max_width
                {
                    self.push_str("\n");
                    self.pad_to_column(target_level);
                    line_start = true; // Needed for next loop
                } else if !line_start {
                    // Add a space between words if we aren't at the start of a wrapped line
                    self.push_str(" ");
                }

                self.push_str(word);
                line_start = false;
            }
            self.push_str("\n");
            self.pad_to_column(target_level);
        }
    }
}

impl fmt::Write for TermWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}
