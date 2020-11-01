use crate::key_value::KeyValue;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{bracketed, token, Token};

/// Holds an outer attribute filled with key-value options
#[derive(Eq, PartialEq, Debug, Default)]
pub struct OptionsAttribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub options: Punctuated<KeyValue, Token![,]>,
}

/// Make OptionsAttribute parsable from a token stream
impl Parse for OptionsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(OptionsAttribute {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            options: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse_options_attribute() -> Result {
        let actual: OptionsAttribute =
            parse_str("#[tmpl = {trait To {};}, no_default, other = Top]")?;
        let mut expected = OptionsAttribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            options: Punctuated::new(),
        };

        expected.options.push(parse_str("tmpl = {trait To {};}")?);
        expected.options.push(parse_str("no_default")?);
        expected.options.push(parse_str("other = Top")?);

        assert_eq!(actual, expected);
        Ok(())
    }
}
