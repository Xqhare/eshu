use eshu::{Cli, CliCmd, CliFlag, StoreSyntax, StoreType};
use std::rc::Rc;

#[test]
fn test_make_manpage_simple() {
    let cli = Cli::new("test-cli")
        .with_version("1.2.3")
        .with_about("A simple test CLI program.")
        .with_author_and_publish_date(
            "Jane Doe <jane@example.com>".to_string(),
            "2026-06-16".to_string(),
        )
        .add_flag(
            CliFlag::new("verbose")
                .with_flag_char('v')
                .with_about(
                    "verbose output",
                    "Print extra verbose details when executing.",
                )
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("file")
                .with_flag_char('f')
                .with_about("file input", "Path to file.")
                .with_required_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("config")
                .with_about("config option", "Path to config.")
                .with_required_store(StoreType::KeyValue, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .add_command(Rc::new(
            CliCmd::new("status")
                .with_about("Show status", "Shows the current status of the program.")
                .add_flag(
                    CliFlag::new("detailed")
                        .with_flag_char('d')
                        .with_about("Detailed output", "Provide more detailed output.")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        ))
        .try_parse_custom(vec!["test-cli".to_string()])
        .unwrap();

    let manpage = cli.make_manpage();
    println!("--- MANPAGE START ---\n{}\n--- MANPAGE END ---", manpage);

    // Check main sections exist
    assert!(manpage.contains(".TH test-cli 1 \"2026-06-16\" \"1.2.3\" \"User Commands\""));
    assert!(manpage.contains(".SH NAME\ntest-cli"));
    assert!(manpage.contains(".SH DESCRIPTION\nA simple test CLI program."));
    assert!(manpage.contains(".SH FLAGS"));

    // Check flags are formatted correctly
    assert!(manpage.contains("\\fB\\-v\\fR, \\fB\\-\\-verbose\\fR"));
    assert!(manpage.contains("verbose output Print extra verbose details when executing."));

    // Check flags with stores are formatted correctly
    assert!(manpage.contains("\\fB\\-f\\fR, \\fB\\-\\-file\\fR \\fIVALUE\\fR"));
    assert!(manpage.contains("\\fB\\-\\-config\\fR=\\fIKEY=VALUE\\fR"));

    // Check subcommand and its nesting/flags are formatted correctly
    assert!(manpage.contains(".SH COMMANDS"));
    assert!(manpage.contains("\\fBstatus\\fR"));
    assert!(manpage.contains("Show status Shows the current status of the program."));
    assert!(manpage.contains(".RS\n.SS FLAGS"));
    assert!(manpage.contains("\\fB\\-d\\fR, \\fB\\-\\-detailed\\fR"));
    assert!(manpage.contains(".RE"));

    // Check author
    assert!(manpage.contains(".SH AUTHOR\nJane Doe <jane@example.com>"));
}
