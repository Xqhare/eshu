use eshu::{Cli, CliCmd, CliFlag};

#[test]
fn simple_cmd() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_command(std::rc::Rc::new(
            CliCmd::new("cmd")
                .with_about("short about", "long about")
                .add_flag(
                    CliFlag::new("a-flag")
                        .with_about("short about", "long")
                        .build()
                        .expect("Failed to build flag"),
                )
                .build()
                .expect("Failed"),
        ))
        .try_parse_custom(vec![
            "test-cli".to_string(),
            "cmd".to_string(),
            "--a-flag".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli: Cli = cli.unwrap();
    assert!(!cli.is_flag_entered("a-flag"));
    assert!(cli.is_subcommand_entered("cmd"));
    let inner_cli = cli.get_subcommand_cli("cmd");
    assert!(inner_cli.is_some());
    let inner_cli = inner_cli.unwrap();
    assert!(inner_cli.is_flag_entered("a-flag"));
}

#[test]
fn simple_cmd_empty_input() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_command(std::rc::Rc::new(
            CliCmd::new("cmd")
                .with_about("short about", "long about")
                .add_flag(
                    CliFlag::new("a-flag")
                        .with_about("short about", "long")
                        .build()
                        .expect("Failed to build flag"),
                )
                .build()
                .expect("Failed"),
        ))
        .try_parse_custom(vec!["test-cli".to_string(), "".to_string()]);
    assert!(cli.is_ok());
    let cli: Cli = cli.unwrap();
    assert!(!cli.is_flag_entered("a-flag"));
    assert!(!cli.is_subcommand_entered("cmd"));
}

#[test]
fn nested_subcommand() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_command(std::rc::Rc::new(
            CliCmd::new("parent")
                .with_about("parent command", "parent command long")
                .add_subcommand(std::rc::Rc::new(
                    CliCmd::new("child")
                        .with_about("child command", "child command long")
                        .add_flag(
                            CliFlag::new("flag")
                                .with_about("flag", "flag")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap(),
        ))
        .try_parse_custom(vec![
            "test-cli".to_string(),
            "parent".to_string(),
            "child".to_string(),
            "--flag".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_subcommand_entered("parent"));

    let parent_cli = cli.get_subcommand_cli("parent").unwrap();
    assert!(parent_cli.is_subcommand_entered("child"));

    let child_cli = parent_cli.get_subcommand_cli("child").unwrap();
    assert!(child_cli.is_flag_entered("flag"));

    // Verify isolation: parent should not have parsed the child flag
    assert!(!parent_cli.is_flag_entered("flag"));
}

#[test]
fn unrecognized_subcommand() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_command(std::rc::Rc::new(
            CliCmd::new("cmd")
                .with_about("cmd", "cmd")
                .add_flag(CliFlag::new("flag").with_about("f", "f").build().unwrap())
                .build()
                .unwrap(),
        ))
        .try_parse_custom(vec!["test-cli".to_string(), "unknown".to_string()]);

    assert!(cli.is_err());
    let err = cli.unwrap_err();
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::UnknownArgument(cmd) => {
            assert_eq!(cmd, "unknown");
        }
        _ => panic!("Expected EshuErrorKind::UnknownArgument"),
    }
}
