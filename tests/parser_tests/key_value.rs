use eshu::{Cli, CliFlag, EshuErrorKind, StoreSyntax, StoreType};

#[test]
fn test_key_value_detached() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("config")
                .with_flag_char('c')
                .with_about("config", "Config flags")
                .with_required_store(StoreType::KeyValue, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "--config".to_string(),
            "host=localhost".to_string(),
            "-c".to_string(),
            "port=8080".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("config"));
    let store = cli
        .get_flag_store("config")
        .unwrap()
        .as_key_value()
        .unwrap();
    assert_eq!(store.get("host").unwrap(), "localhost");
    assert_eq!(store.get("port").unwrap(), "8080");
}

#[test]
fn test_key_value_attached() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("config")
                .with_flag_char('c')
                .with_about("config", "Config flags")
                .with_required_store(StoreType::KeyValue, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "--config=host=localhost".to_string(),
            "-c=port=8080".to_string(),
        ]);

    assert!(cli.is_ok());
    let cli = cli.unwrap();
    let store = cli
        .get_flag_store("config")
        .unwrap()
        .as_key_value()
        .unwrap();
    assert_eq!(store.get("host").unwrap(), "localhost");
    assert_eq!(store.get("port").unwrap(), "8080");
}

#[test]
fn test_key_value_malformed() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("config")
                .with_about("config", "Config flags")
                .with_required_store(StoreType::KeyValue, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec![
            "test".to_string(),
            "--config".to_string(),
            "malformed_no_equals".to_string(),
        ]);

    assert!(cli.is_err());
    let err = cli.unwrap_err();
    let leaf_err = err.downcast_ref::<EshuErrorKind>().unwrap();
    match leaf_err {
        EshuErrorKind::MissingArgument {
            flag,
            expected_syntax,
        } => {
            assert_eq!(flag, "-config (--config)");
            assert_eq!(expected_syntax, "-key=value");
        }
        _ => panic!("Expected MissingArgument for malformed key-value pair"),
    }
}
