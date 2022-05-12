//! Define [`Command`] line [arguments][`Arg`]

#[macro_use]
mod macros;

mod app_settings;
mod arg;
mod arg_group;
mod arg_predicate;
mod arg_settings;
mod command;
mod possible_value;
mod usage_parser;
mod value_hint;
mod value_parser;

#[cfg(feature = "regex")]
mod regex;

#[cfg(debug_assertions)]
mod debug_asserts;

#[cfg(test)]
mod tests;

pub use app_settings::{AppFlags, AppSettings};
pub use arg::Arg;
pub use arg_group::ArgGroup;
pub use arg_settings::{ArgFlags, ArgSettings};
pub use command::Command;
pub use possible_value::PossibleValue;
pub use value_hint::ValueHint;
pub use value_parser::AnyValueParser;
pub use value_parser::AutoValueParser;
pub use value_parser::TypedValueParser;
pub use value_parser::ValueParser;
pub use value_parser::ValueParserViaBuiltIn;
pub use value_parser::ValueParserViaFromStr;

#[allow(deprecated)]
pub use command::App;

#[cfg(feature = "regex")]
pub use self::regex::RegexRef;

pub(crate) use arg::display_arg_val;
pub(crate) use arg_predicate::ArgPredicate;
pub(crate) use value_parser::ValueParserInner;
