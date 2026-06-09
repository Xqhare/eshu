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
        .parse_custom(vec!["test".to_string(), "--a-flag".to_string()]);
    assert!(cli.is_ok());
    let cli = cli.unwrap();
    assert!(cli.is_flag_entered("a-flag"));
}
