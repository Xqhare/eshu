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
        .parse_custom(vec![
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
        .parse_custom(vec!["test-cli".to_string(), "".to_string()]);
    assert!(cli.is_ok());
    let cli: Cli = cli.unwrap();
    assert!(!cli.is_flag_entered("a-flag"));
    assert!(!cli.is_subcommand_entered("cmd"));
}
