use std::{collections::BTreeMap, iter::Peekable, str::Chars};

use crate::{
    CliFlag, EshuErrorKind, StoreSyntax, StoreType,
    cli::builder::CliBuilder,
    utils::{Store, is_positional},
};
use nemesis::NemesisError;

/// Handles grouped flags
#[expect(clippy::shadow_unrelated, reason = "Shadowing is fine here")]
#[expect(clippy::too_many_lines, reason = "Parsing is complex")]
#[expect(clippy::type_complexity, reason = "Parsing is complex")]
pub fn parse_grouped_flags(
    arg: &str,
    cli_builder: &CliBuilder,
    next_arg: Option<&str>,
    detached_list_args: Option<&[String]>,
) -> Result<Vec<(String, (usize, Store))>, NemesisError> {
    let mut out: Vec<(String, (usize, Store))> = Vec::with_capacity(arg.len()); // Should always over-allocate

    let args = arg.chars().collect::<Vec<char>>();
    let mut arg_iter = arg.chars().peekable();
    let mut index: usize = 0;

    while let Some(c) = arg_iter.next() {
        if index == 0 && c == '-' {
            index = index.saturating_add(1);
            continue;
        }
        index = index.saturating_add(1);

        if c.is_ascii_alphabetic() {
            let mut matched_flag = None;
            for flag in &cli_builder.flags {
                if flag.flag_char == Some(c) {
                    matched_flag = Some(flag);
                    break;
                }
            }
            if let Some(flag) = matched_flag {
                let stored_value = get_flag_store(
                    flag,
                    &mut arg_iter,
                    &args,
                    index,
                    arg,
                    next_arg,
                    c,
                    detached_list_args,
                )?;
                if stored_value.is_none() && flag.required_store {
                    return Err(NemesisError::new(
                        "eshu::parser",
                        EshuErrorKind::MissingArgument {
                            flag: format!("-{} (--{})", c, flag.long_flag),
                            expected_syntax: format!("-{}={}", c, "VALUE"),
                        },
                    ));
                }
                if let Some(stored_value) = stored_value {
                    if stored_value.len() <= 1 {
                        let stored_value = stored_value.first().unwrap_or(&String::new()).clone();
                        match flag.store_type {
                            Some(StoreType::Value) => {
                                out.push((
                                    flag.long_flag.clone(),
                                    (index, Store::Value(vec![stored_value])),
                                ));
                                break;
                            }
                            Some(StoreType::KeyValue) => {
                                let (key, val) =
                                    if let Some((key, val)) = stored_value.split_once('=') {
                                        (key, val)
                                    } else {
                                        return Err(NemesisError::new(
                                            "eshu::parser",
                                            EshuErrorKind::MissingArgument {
                                                flag: format!("-{} (--{})", c, flag.long_flag),
                                                expected_syntax: "-key=value".to_string(),
                                            },
                                        ));
                                    };
                                out.push((
                                    flag.long_flag.clone(),
                                    (
                                        index,
                                        Store::KeyValue(BTreeMap::from([(
                                            key.to_string(),
                                            val.to_string(),
                                        )])),
                                    ),
                                ));
                                break;
                            }
                            None => {
                                return Err(NemesisError::new(
                                    "eshu::parser",
                                    EshuErrorKind::Generic(format!(
                                        "Flag '{}' does not have a store type but an associated value was found: '{}'",
                                        flag.long_flag, stored_value
                                    )),
                                ));
                            }
                        }
                    } else {
                        match flag.store_type {
                            Some(StoreType::Value) => {
                                out.push((
                                    flag.long_flag.clone(),
                                    (index, Store::Value(stored_value)),
                                ));
                            }
                            Some(StoreType::KeyValue) => {
                                let mut map = BTreeMap::new();
                                for arg in stored_value {
                                    let (key, val) = if let Some((key, value)) = arg.split_once('=')
                                    {
                                        (key, value)
                                    } else {
                                        return Err(NemesisError::new(
                                            "eshu::parser",
                                            EshuErrorKind::MissingArgument {
                                                flag: format!("-{} (--{})", arg, flag.long_flag),
                                                expected_syntax: "-key=value".to_string(),
                                            },
                                        ));
                                    };
                                    map.insert(key.to_string(), val.to_string());
                                }
                                out.push((flag.long_flag.clone(), (index, Store::KeyValue(map))));
                            }
                            None => {
                                // Don't do anything
                            }
                        }
                    }
                } else {
                    match flag.store_type {
                        Some(StoreType::Value) => {
                            out.push((flag.long_flag.clone(), (index, Store::Value(vec![]))));
                        }
                        Some(StoreType::KeyValue) => {
                            out.push((
                                flag.long_flag.clone(),
                                (index, Store::KeyValue(BTreeMap::new())),
                            ));
                        }
                        None => {
                            out.push((flag.long_flag.clone(), (index, Store::Exists)));
                        }
                    }
                }
            } else {
                return Err(NemesisError::new(
                    "eshu::parser",
                    EshuErrorKind::Generic(format!("Flag character '{c}' not found")),
                ));
            }
        } else {
            return Err(NemesisError::new(
                "eshu::parser",
                EshuErrorKind::Generic(format!("Invalid flag character: {c}")),
            ));
        }
    }

    Ok(out)
}

#[expect(clippy::shadow_unrelated, reason = "Shadowing is fine here")]
#[expect(clippy::too_many_arguments, reason = "Parsing is complex")]
fn get_flag_store(
    flag: &CliFlag,
    arg_iter: &mut Peekable<Chars>,
    args: &[char],
    index: usize,
    arg: &str,
    next_arg: Option<&str>,
    c: char,
    detached_list_args: Option<&[String]>,
) -> Result<Option<Vec<String>>, NemesisError> {
    let mut stored_value: Option<Vec<String>> = None;
    if flag.storing {
        match flag.store_syntax {
            Some(StoreSyntax::Attached) => {
                if let Some(next_arg) = arg_iter.peek()
                    && next_arg == &'='
                {
                    stored_value = Some(vec![
                        args[index.saturating_add(1)..].iter().collect::<String>(),
                    ]);
                }
                if stored_value.is_none() && flag.required_store {
                    if let Some(detached_list_args) = detached_list_args {
                        return Ok(Some(detached_list_args.to_vec()));
                    }
                    if index == arg.len() {
                        return Err(NemesisError::new(
                            "eshu::parser",
                            EshuErrorKind::MissingArgument {
                                flag: format!("-{} (--{})", c, flag.long_flag),
                                expected_syntax: format!("-{}={}", c, "VALUE"),
                            },
                        ));
                    }
                    // POSIX, as I understand it, requires using all following chars of a
                    // grouped flag as the value if the flag accepts values. Horrible way of
                    // putting it
                    stored_value = Some(vec![
                        args[index.saturating_add(1)..].iter().collect::<String>(),
                    ]);
                }
            }
            Some(StoreSyntax::Detached) => {
                // TODO: add to the doc that only the *last* flag (in the entire group) can have a detached value
                if index == arg.len()
                    && let Some(next_arg) = next_arg
                    && is_positional(next_arg)
                {
                    stored_value = Some(vec![next_arg.to_string()]);
                }
                if flag.required_store && stored_value.is_none() {
                    if let Some(detached_list_args) = detached_list_args {
                        return Ok(Some(detached_list_args.to_vec()));
                    }
                    return Err(NemesisError::new(
                        "eshu::parser",
                        EshuErrorKind::MissingArgument {
                            flag: format!("-{} (--{})", c, flag.long_flag),
                            expected_syntax: format!("-{} {}", c, "VALUE"),
                        },
                    ));
                }
            }
            None => {}
        }
    }

    Ok(stored_value)
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
    let out = parse_grouped_flags("-abc", &cli, None, None).unwrap();
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-abs=1", &cli, None, None).unwrap();
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-abc", &cli, None, None).unwrap();
    assert_eq!(out.len(), 3);
    let out = parse_grouped_flags("-ao=1", &cli, None, None).unwrap();
    assert_eq!(out.len(), 2);
    let out = parse_grouped_flags("-ao", &cli, None, None).unwrap();
    assert_eq!(out.len(), 2);
    let out = parse_grouped_flags("-ar=1", &cli, None, None).unwrap();
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
    let out = parse_grouped_flags("-a", &cli, Some("1"), None).unwrap();
    assert_eq!(out.len(), 1);
}
