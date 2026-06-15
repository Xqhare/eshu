use eshu::{Cli, CliFlag};

#[test]
fn test_long_flag_basic() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec!["test".to_string(), "--a-flag".to_string()]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
}

#[test]
fn partial_flags() {
    use eshu::{StoreSyntax, StoreType};
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
                .with_about("Port", "An optional port")
                .with_required_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("port-other")
                .with_flag_char('o')
                .with_about("Port", "An optional port")
                .with_required_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("boolean")
                .with_flag_char('b')
                .with_about("Boolean", "Testing bool")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "--port-oth".to_string(),
            "8080".to_string(),
            "--port".to_string(),
            "0123".to_string(),
            "--bo".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("port"));
    assert_eq!(
        cli.get_flag_store("port").unwrap().as_value().unwrap(),
        &vec!["0123".to_string()]
    );
    assert!(cli.is_flag_entered("port-other"));
    assert_eq!(
        cli.get_flag_store("port-other")
            .unwrap()
            .as_value()
            .unwrap(),
        &vec!["8080".to_string()]
    );
    assert!(cli.is_flag_entered("boolean"));
}
