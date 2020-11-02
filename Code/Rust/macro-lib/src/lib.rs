pub mod extensions;
mod key_value;
mod options_attribute;
mod rich_type;
mod simple_type;
pub mod token_stream_utils;
mod trait_specifier;

pub use key_value::KeyValue;
pub use options_attribute::OptionsAttribute;
pub use rich_type::RichType;
pub use simple_type::SimpleType;
pub use trait_specifier::TraitSpecifier;
// TODO: consider if some Punctuated::parse_terminated should no be Punctuated::parse_seperated_nonempty
