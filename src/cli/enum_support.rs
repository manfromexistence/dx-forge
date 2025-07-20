use std::str::FromStr;

use crate::cli::{
    error::InquireResult,
    // list_option::{FromListOption, ListOption},
    list_option::{ListOption},
};

pub trait InquireEnumVariants {
    fn get_variants() -> &'static [&'static str];
}

#[cfg(feature = "strum")]
impl<T> InquireEnumVariants for T
where
    T: strum::VariantNames,
{
    fn get_variants() -> &'static [&'static str] {
        T::VARIANTS
    }
}
