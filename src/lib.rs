#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::restriction)]
#![expect(
    clippy::missing_docs_in_private_items,
    clippy::print_stdout,
    clippy::implicit_return,
    clippy::single_call_fn,
    clippy::str_to_string,
    clippy::question_mark_used,
    clippy::indexing_slicing,
    clippy::pattern_type_mismatch,
    clippy::arbitrary_source_item_ordering,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::exhaustive_enums,
    clippy::min_ident_chars,
    clippy::missing_trait_methods,
    clippy::impl_trait_in_params,
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::shadow_reuse,
    clippy::blanket_clippy_restriction_lints,
    clippy::doc_include_without_cfg,
    reason = "Ignored warnings"
)]

// For 1.0.0:
// TODO: Add tests
// TODO: Add examples
// TODO: Make clippy happy
// TODO: Make safe:
//      TODO: Sweep for expects, write_err_and_exit, unwrap

mod arg_parser;
mod cli;
mod cli_cmd;
mod error;
mod utils;

pub use cli::Cli;
pub use cli_cmd::*;
pub use error::EshuError;
