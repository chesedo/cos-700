pub mod extensions;
pub mod key_value;
pub mod options_attribute;
mod rich_type;
mod simple_type;
pub mod token_stream_utils;
pub mod trait_specifier;

pub use rich_type::RichType;
pub use simple_type::SimpleType;
// TODO: consider if some Punctuated::parse_terminated should no be Punctuated::parse_seperated_nonempty
