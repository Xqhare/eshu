use std::collections::BTreeMap;

use crate::{
    StoreSyntax, StoreType,
    cli::builder::CliBuilder,
    utils::{Store, is_positional, write_err_and_exit},
};

/// Handles grouped flags
pub fn parse_grouped_flags(
    arg: &str,
    cli_builder: &CliBuilder,
    next_arg: Option<&str>,
) -> Vec<(String, (usize, Store))> {
    let mut value = None;
    let mut storing: Vec<char> = Vec::new();
    for (i, c) in arg.char_indices() {
        if i == 0 && c == '-' {
            continue;
        }
        if c == '=' {
            value = Some(arg[i + 1..].to_string());
            break;
        }
        if c.is_ascii_alphabetic() {
            storing.push(c);
        } else {
            write_err_and_exit(&format!("Usage error: Flag must be a-z/A-Z. Got: {}", arg));
        }
    }

    let mut out: Vec<(String, (usize, Store))> = Vec::new();

    for (index, c) in storing.iter().enumerate() {
        for (i, flag) in cli_builder.flags.iter().enumerate() {
            if flag.flag_char == Some(*c) {
                if index == storing.len() - 1 {
                    if flag.storing && value.is_none() {
                        if flag.store_syntax == Some(StoreSyntax::Detached) {
                            if let Some(next_arg) = next_arg {
                                if is_positional(next_arg) {
                                    value = Some(next_arg.to_string());
                                } else if flag.required_store {
                                    write_err_and_exit(&format!(
                                        "Usage error: Flag {} requires an argument. Not attached value found, detached value found '{}' is not a positional argument.\n\nPlease provide one via the following syntax: '-{} VALUE' or '-{}=VALUE' ",
                                        flag.long_flag, next_arg, *c, *c
                                    ));
                                }
                            }
                        }
                    }
                    if flag.required_store && value.is_none() {
                        let req_syntax = match &flag.store_syntax.expect("Store syntax not set") {
                            StoreSyntax::Attached => &format!("-{}={}", *c, "VALUE"),
                            StoreSyntax::Detached => &format!("-{} {}", *c, "VALUE"),
                        };
                        write_err_and_exit(&format!(
                            "Usage error: Flag '-{}' (--{}) requires an argument. Please provide one via the following syntax: '{}'",
                            *c, flag.long_flag, req_syntax
                        ));
                    }
                    if let Some(value) = value.clone() {
                        if flag.store_type.is_none() {
                            write_err_and_exit(&format!(
                                "Usage error: Flag {} does not take a value. Eshu found the following value passed to it: {}",
                                flag.long_flag, value
                            ));
                        }
                        match flag.store_type.unwrap() {
                            StoreType::Value => {
                                out.push((flag.long_flag.clone(), (i, Store::Value(vec![value]))))
                            }
                            StoreType::KeyValue => {
                                let split = value.split_once('=');
                                if split.is_none() {
                                    write_err_and_exit(&format!(
                                        "Usage error: Expected key=value, got: {}",
                                        value
                                    ));
                                }
                                let (k, v) = split.unwrap();
                                out.push((
                                    flag.long_flag.clone(),
                                    (
                                        i,
                                        Store::KeyValue(BTreeMap::from([(
                                            k.to_string(),
                                            v.to_string(),
                                        )])),
                                    ),
                                ))
                            }
                        }
                    } else {
                        out.push((flag.long_flag.clone(), (i, Store::Exists)));
                    }
                } else {
                    out.push((flag.long_flag.clone(), (i, Store::Exists)));
                }
            }
        }
    }

    out
}

#[test]
fn grouped_flags_attached() {
    use crate::CliFlag;
    let cli = CliBuilder::new("test")
        .add_flag(
            CliFlag::new("a-flag")
                .with_about("test", "test")
                .with_flag_char('a')
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("b-flag")
                .with_about("test", "test")
                .with_flag_char('b')
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("c-flag")
                .with_about("test", "test")
                .with_flag_char('c')
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("storing-flag")
                .with_about("test", "test")
                .with_flag_char('s')
                .with_required_store(StoreType::Value, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("optional-storing-flag")
                .with_about("test", "test")
                .with_flag_char('o')
                .with_store(StoreType::Value, StoreSyntax::Attached)
                .build()
                .unwrap(),
        )
        .add_flag(
            CliFlag::new("required-storing-flag")
                .with_about("test", "test")
                .with_flag_char('r')
                .with_required_store(StoreType::Value, StoreSyntax::Attached)
                .build()
                .unwrap(),
        );
    let out = parse_grouped_flags("-abc", &cli, None);
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-abs=1", &cli, None);
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-abc", &cli, None);
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-ao=1", &cli, None);
    assert_eq!(out.len(), 2);
    let out = parse_grouped_flags("-ao", &cli, None);
    assert_eq!(out.len(), 2);
    let out = parse_grouped_flags("-ar=1", &cli, None);
    assert_eq!(out.len(), 2);
}

#[test]
fn single_flag_detached() {
    use crate::CliFlag;
    let cli = CliBuilder::new("test").add_flag(
        CliFlag::new("a-flag")
            .with_about("test", "test")
            .with_flag_char('a')
            .with_required_store(StoreType::Value, StoreSyntax::Detached)
            .build()
            .unwrap(),
    );
    let out = parse_grouped_flags("-a", &cli, Some("1"));
    assert_eq!(out.len(), 1);
}
