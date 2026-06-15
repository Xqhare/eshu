use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

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

#[test]
fn partial_flags() {
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

#[test]
fn regression_detached_store() {
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
            "--port-other".to_string(),
            "8080".to_string(),
            "--port".to_string(),
            "0123".to_string(),
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
}

#[test]
fn regression_attached_store() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
                .with_about("Port", "An optional port")
                .with_required_store(StoreType::Value, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("port-other")
                .with_flag_char('o')
                .with_about("Port", "An optional port")
                .with_required_store(StoreType::Value, StoreSyntax::Attached)
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
            "--port-other=8080".to_string(),
            "--port=0123".to_string(),
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
}
#[test]
fn regression_single_short_flag_overwrite() {
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
            CliFlag::new("boolean")
                .with_flag_char('b')
                .with_about("Boolean", "Testing bool")
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "-p".to_string(),
            "8080".to_string(),
            "-p".to_string(),
            "0123".to_string(),
            "-bp".to_string(),
            "4567".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("port"));
    assert_eq!(
        cli.get_flag_store("port").unwrap().as_value().unwrap(),
        &vec!["8080".to_string(), "0123".to_string(), "4567".to_string()]
    );
}

#[test]
fn regression_optional_store_no_value() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_flag_char('a')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .with_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
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
            "-a".to_string(),
            "detachedValue".to_string(),
            "-p".to_string(),
            "8080".to_string(),
            "--port".to_string(),
            "0420".to_string(),
            "-p".to_string(),
            "8081".to_string(),
            "-bp".to_string(),
            "1234".to_string(),
            "-a".to_string(),
            "--port".to_string(),
            "5678".to_string(),
            "-bap".to_string(),
            "9510".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert_eq!(
        cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
        &vec!["detachedValue".to_string()]
    );
    assert!(cli.is_flag_entered("port"));
    assert_eq!(
        cli.get_flag_store("port").unwrap().as_value().unwrap(),
        &vec![
            "8080".to_string(),
            "0420".to_string(),
            "8081".to_string(),
            "1234".to_string(),
            "5678".to_string(),
            "9510".to_string(),
        ]
    );
}

#[test]
fn detached_store() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_flag_char('a')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .with_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
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
            "-a".to_string(),
            "detachedValue".to_string(),
            "-p".to_string(),
            "8080".to_string(),
            "--port".to_string(),
            "0420".to_string(),
            "-p".to_string(),
            "8081".to_string(),
            "-bp".to_string(),
            "1234".to_string(),
            "-a".to_string(),
            "--port".to_string(),
            "5678".to_string(),
            "-bap".to_string(),
            "9510".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert_eq!(
        cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
        &vec!["detachedValue".to_string()]
    );
    assert!(cli.is_flag_entered("boolean"));
    assert!(cli.get_flag_store("boolean").unwrap().exists());
    assert!(cli.is_flag_entered("port"));
    assert_eq!(
        cli.get_flag_store("port").unwrap().as_value().unwrap(),
        &vec![
            "8080".to_string(),
            "0420".to_string(),
            "8081".to_string(),
            "1234".to_string(),
            "5678".to_string(),
            "9510".to_string(),
        ]
    );
}

#[test]
fn attached_store() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_flag_char('a')
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .with_store(StoreType::Value, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
                .with_about("Port", "An optional port")
                .with_required_store(StoreType::Value, StoreSyntax::Attached)
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
            "--a-flag=AttachedValue".to_string(),
            "-p=8080".to_string(),
            "--port=0420".to_string(),
            "-p=8081".to_string(),
            "-b".to_string(),
            "-bp=1234".to_string(),
            "-a".to_string(),
            "--port=5678".to_string(),
            "-bap=9510".to_string(),
        ]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
    assert_eq!(
        cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
        &vec!["AttachedValue".to_string()]
    );
    assert!(cli.is_flag_entered("boolean"));
    assert!(cli.get_flag_store("boolean").unwrap().exists());
    assert!(cli.is_flag_entered("port"));
    assert_eq!(
        cli.get_flag_store("port").unwrap().as_value().unwrap(),
        &vec![
            "8080".to_string(),
            "0420".to_string(),
            "8081".to_string(),
            "1234".to_string(),
            "5678".to_string(),
            "9510".to_string(),
        ]
    );
}
