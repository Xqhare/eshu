use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

#[test]
fn build_most_basic_cli() {
    let cli = Cli::new("test-cli")
        .with_version("0.0.0")
        .basic()
        .parse_custom(vec!["test".to_string()]);
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
        .parse_custom(vec!["test".to_string()]);
    assert!(cli.is_ok());
}

#[test]
fn cli_requirements() {
    let no_flag = Cli::new("test-cli")
        .with_version("0.0.0")
        .parse_custom(vec!["test".to_string()]);
    assert!(no_flag.is_err());
    let no_ver = Cli::new("test-cli")
        .add_flag(
            CliFlag::new("a-flag")
                .with_short_about("Testing text")
                .with_long_about("Long testing text \n lorem ipsum")
                .build()
                .unwrap(),
        )
        .parse_custom(vec!["test".to_string()]);
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
        .parse_custom(vec!["test".to_string()]);
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

