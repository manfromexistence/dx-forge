#![warn(missing_docs)]
#![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::bool_to_int_with_if)]
#[cfg(feature = "date")]
mod ansi;
mod config;
mod date_utils;
mod input;
mod prompts;
mod terminal;
mod utils;
pub mod autocompletion;
pub mod error;
pub mod formatter;
pub mod list_option;
pub mod parser;
pub mod type_aliases;
pub mod ui;
pub mod validator;

pub use crate::autocompletion::Autocomplete;
pub use crate::config::set_global_render_config;
pub use crate::error::{CustomUserError, InquireError};
pub use crate::input::action::*;
pub use crate::prompts::*;
