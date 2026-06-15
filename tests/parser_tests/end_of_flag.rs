use eshu::{Cli, CliFlag};

#[test]
fn end_of_flag_marker_no_store_long_flag() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "--a-flag".to_string(),
            "--".to_string(),
            "RougeArgument".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert_eq!(
        cli.get_stray_positional_args(),
        &vec!["RougeArgument".to_string()]
    );
}

#[test]
fn end_of_flag_marker_no_store_short_flag() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_flag_char('a')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "-a".to_string(),
            "--".to_string(),
            "RougeArgument".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert_eq!(
        cli.get_stray_positional_args(),
        &vec!["RougeArgument".to_string()]
    );
}

#[test]
fn end_of_flag_marker_no_store_group_flag() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_flag_char('a')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("b-flag")
                .with_flag_char('b')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "-ba".to_string(),
            "--".to_string(),
            "RougeArgument".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert!(cli.is_flag_entered("b-flag"));
    assert_eq!(
        cli.get_stray_positional_args(),
        &vec!["RougeArgument".to_string()]
    );
}
