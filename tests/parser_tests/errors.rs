use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

#[test]
fn test_missing_argument_long_flag() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("port")
                .with_about("Port", "Port flag")
                .with_required_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec!["test".to_string(), "--port".to_string()]);
    assert!(cli.is_err());
    let err = cli.err().unwrap();
    assert_eq!(err.source_name(), "eshu::parser");
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::MissingArgument {
            flag,
            expected_syntax,
        } => {
            assert_eq!(flag, "--port");
            assert_eq!(expected_syntax, "--port VALUE");
        }
        _ => panic!("Expected EshuErrorKind::MissingArgument"),
    }
}

#[test]
fn test_missing_argument_short_flag() {
    let cli = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("port")
                .with_flag_char('p')
                .with_about("Port", "Port flag")
                .with_required_store(StoreType::Value, StoreSyntax::Detached)
                .build()
                .unwrap(),
        )
        .with_version("0.0.0")
        .try_parse_custom(vec!["test".to_string(), "-p".to_string()]);
    assert!(cli.is_err());
    let err = cli.err().unwrap();
    assert_eq!(err.source_name(), "eshu::parser");
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::MissingArgument {
            flag,
            expected_syntax,
        } => {
            assert_eq!(flag, "-p (--port)");
            assert_eq!(expected_syntax, "-p VALUE");
        }
        _ => panic!("Expected EshuErrorKind::MissingArgument"),
    }
}
