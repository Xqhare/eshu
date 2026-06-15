use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

#[test]
fn build_most_basic_cli() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .basic()
        .try_parse_custom(vec!["test".to_string()]);
    assert!(cli.is_ok());
}

#[test]
fn build_basic_cli() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .try_parse_custom(vec!["test".to_string()]);
    assert!(cli.is_ok());
}

#[test]
fn cli_requirements() {
    let no_flag = Cli::new("test-cli")
        .with_version("0.0.0")
        .try_parse_custom(vec!["test".to_string()]);
    assert!(no_flag.is_err());
    let no_ver = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .try_parse_custom(vec!["test".to_string()]);
    assert!(no_ver.is_err());
    let no_name = Cli::new("")
        .with_version("0.0.0")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .try_parse_custom(vec!["test".to_string()]);
    assert!(no_name.is_err());
}

#[test]
fn build_most_basic_flag() {
    let flag = CliFlag::new("a-flag")
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(flag.is_ok());
}

#[test]
fn build_basic_flag() {
    let flag = CliFlag::new("a-flag")
        .with_flag_char('a')
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(flag.is_ok());
}

#[test]
fn build_complete_flag() {
    let flag = CliFlag::new("a-flag")
        .with_flag_char('a')
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .with_required_store(StoreType::Value, StoreSyntax::Attached)
        .build();
    assert!(flag.is_ok());
}

#[test]
fn flag_requirements() {
    let no_name = CliFlag::new("")
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(no_name.is_err());
    let no_short = CliFlag::new("a-flag")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(no_short.is_err());
    let no_long = CliFlag::new("a-flag")
        .with_short_about("Testing text")
        .build();
    assert!(no_long.is_err());
    let starts_with_dash = CliFlag::new("-a-flag")
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(starts_with_dash.is_err());
    let starts_with_double_dash = CliFlag::new("--a-flag")
        .with_short_about("Testing text")
        .with_long_about("Long testing text \n lorem ipsum")
        .build();
    assert!(starts_with_double_dash.is_ok());
}

#[test]
fn test_programmatic_downcast() {
    let err = match Cli::new("") // Invalid empty name
        .with_version("0.0.0")
        .try_parse_custom(vec!["test".to_string()])
    {
        Ok(_) => panic!("Expected error, got Ok"),
        Err(e) => e,
    };
    
    assert_eq!(err.source_name(), "eshu::builder");
    if let Some(eshu_err) = err.downcast_ref::<eshu::EshuErrorKind>() {
        match eshu_err {
            eshu::EshuErrorKind::EmptyString(msg) => assert!(msg.contains("Name")),
            _ => panic!("Expected EmptyString error"),
        }
    } else {
        panic!("Failed to downcast leaf error");
    }
}

#[test]
fn duplicate_flag_name_error() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_flag(
            CliFlag::new("flag")
                .with_about("f", "f")
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("flag")
                .with_about("duplicate", "duplicate")
                .build()
                .unwrap(),
        )
        .try_parse_custom(vec!["test-cli".to_string()]);

    assert!(cli.is_err());
    let err = cli.unwrap_err();
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::Duplicate(msg) => {
            assert!(msg.contains("Flag"));
        }
        _ => panic!("Expected EshuErrorKind::Duplicate"),
    }
}

#[test]
fn duplicate_flag_char_error() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .add_flag(
            CliFlag::new("flag1")
                .with_flag_char('f')
                .with_about("f", "f")
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("flag2")
                .with_flag_char('f')
                .with_about("duplicate char", "duplicate char")
                .build()
                .unwrap(),
        )
        .try_parse_custom(vec!["test-cli".to_string()]);

    assert!(cli.is_err());
    let err = cli.unwrap_err();
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::Duplicate(msg) => {
            assert!(msg.contains("Short Flag Char"));
        }
        _ => panic!("Expected EshuErrorKind::Duplicate"),
    }
}

#[test]
fn invalid_flag_name_whitespace() {
    let flag = CliFlag::new("invalid name")
        .with_about("f", "f")
        .build();
    assert!(flag.is_err());
    let err = flag.unwrap_err();
    let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
    match leaf_err {
        eshu::EshuErrorKind::InvalidFlagName(msg) => {
            assert!(msg.contains("whitespace"));
        }
        _ => panic!("Expected EshuErrorKind::InvalidFlagName"),
    }
}



