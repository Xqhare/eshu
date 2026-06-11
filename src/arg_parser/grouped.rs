use std::{collections::BTreeMap, iter::Peekable, str::Chars};

use crate::{
    CliFlag, StoreSyntax, StoreType,
    cli::builder::CliBuilder,
    utils::{Store, is_positional, write_err_and_exit},
};

/// Handles grouped flags
pub fn parse_grouped_flags(
    arg: &str,
    cli_builder: &CliBuilder,
    next_arg: Option<&str>,
) -> Vec<(String, (usize, Store))> {
    let mut out: Vec<(String, (usize, Store))> = Vec::with_capacity(arg.len()); // Should always over-allocate

    let args = arg.chars().collect::<Vec<char>>();
    let mut arg_iter = arg.chars().peekable();
    let mut index: usize = 0;

    while let Some(c) = arg_iter.next() {
        if index == 0 && c == '-' {
            index += 1;
            continue;
        }
        index += 1;

        if c.is_ascii_alphabetic() {
            let mut matched_flag = None;
            for flag in cli_builder.flags.iter() {
                if flag.flag_char == Some(c) {
                    matched_flag = Some(flag);
                    break;
                }
            }
            if let Some(flag) = matched_flag {
                let stored_value =
                    get_flag_store(flag, &mut arg_iter, &args, index, arg, next_arg, c);
                if stored_value.is_none() && flag.required_store {
                    write_err_and_exit(&format!(
                        "Usage error: Flag '-{}' (--{}) requires an argument. Please provide one via the following syntax: '{}'",
                        c,
                        flag.long_flag,
                        format!("-{}={}", c, "VALUE")
                    ))
                }
                if let Some(stored_value) = stored_value {
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
                                stored_value.split_once('=').expect("Must be key=value");
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
                            write_err_and_exit(&format!(
                                "Flag '{}' does not have a store type but an associated value was found: '{}'",
                                flag.long_flag, stored_value
                            ));
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
                write_err_and_exit(&format!("Flag character '{}' not found", c));
            }
        } else {
            write_err_and_exit(&format!("Invalid flag character: {}", c));
        }
    }

    out
}

fn get_flag_store(
    flag: &CliFlag,
    arg_iter: &mut Peekable<Chars>,
    args: &Vec<char>,
    index: usize,
    arg: &str,
    next_arg: Option<&str>,
    c: char,
) -> Option<String> {
    let mut stored_value = None;
    if flag.storing {
        match flag.store_syntax {
            Some(StoreSyntax::Attached) => {
                if let Some(next_arg) = arg_iter.peek() {
                    if next_arg == &'=' {
                        stored_value =
                            Some(args[index.saturating_add(1)..].iter().collect::<String>());
                    }
                }
                if stored_value.is_none() && flag.required_store {
                    if index == arg.len() {
                        write_err_and_exit(&format!(
                            "Usage error: Flag '-{}' (--{}) requires an argument. Please provide one via the following syntax: '{}'",
                            c,
                            flag.long_flag,
                            format!("-{}={}", c, "VALUE")
                        ))
                    };
                    // POSIX, as I understand it, requires using all following chars of a
                    // grouped flag as the value if the flag accepts values. Horrible way of
                    // putting it
                    stored_value = Some(args[index.saturating_add(1)..].iter().collect::<String>());
                }
            }
            Some(StoreSyntax::Detached) => {
                println!("DOG DOG");
                // TODO: add to the doc that only the *last* flag (in the entire group) can have a detached value
                if index == arg.len() {
                    if let Some(next_arg) = next_arg {
                        if is_positional(next_arg) {
                            stored_value = Some(next_arg.to_string());
                        }
                    }
                    if flag.required_store && stored_value.is_none() {
                        write_err_and_exit(&format!(
                            "Usage error: Flag '-{}' (--{}) requires an argument. Please provide one via the following syntax: '{}'",
                            c,
                            flag.long_flag,
                            format!("-{} VALUE", c)
                        ))
                    }
                }
            }
            None => {}
        }
    }

    stored_value
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
