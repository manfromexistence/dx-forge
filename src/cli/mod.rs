#![warn(missing_docs)]
#![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::bool_to_int_with_if)]

mod ansi;
mod date_utils;
mod enum_support;
mod input;
mod terminal;
mod utils;

pub mod autocompletion;
pub mod config;
pub mod error;
pub mod formatter;
pub mod list_option;
pub mod parser;
pub mod prompts;
pub mod type_aliases;
pub mod ui;
pub mod validator;

pub use self::autocompletion::Autocomplete;
pub use self::config::set_global_render_config;
pub use self::error::{CustomUserError, InquireError};
pub use self::input::action::*;
pub use self::prompts::*;