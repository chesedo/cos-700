use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

/// Holds a single key value attribute, with the value being optional
#[derive(Debug)]
pub struct AttributeItem {
    key: Ident,
    equal_token: Token![=],
    value: TokenStream,
}

/// Make AttributeItem parsable from a token stream
impl Parse for AttributeItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;

        // Stop if optional value is not given
        if input.is_empty() || input.peek(Token![,]) {
            return Ok(AttributeItem {
                key,
                equal_token: Default::default(),
                value: Default::default(),
            });
        }

        // Get the equal sign
        let equal = input.parse()?;

        // Get the next token item from the parse stream
        let value = input.step(|cursor| {
            let mut s = TokenStream::new();

            if let Some((inner, rest)) = cursor.token_tree() {
                s.append(inner);
                return Ok((s, rest));
            }

            Err(cursor.error("value not supplied"))
        })?;

        Ok(AttributeItem {
            key,
            equal_token: equal,
            value,
        })
    }
}

// Just for testing
#[cfg(test)]
impl PartialEq for AttributeItem {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && format!("{}", self.value) == format!("{}", other.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse_attribute_item() -> Result {
        let actual: AttributeItem = parse_str("some_key = \"value\"")?;
        let expected = AttributeItem {
            key: parse_str("some_key")?,
            equal_token: Default::default(),
            value: parse_str("\"value\"")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_attribute_item_missing_value() -> Result {
        let actual: AttributeItem = parse_str("bool_key")?;
        let expected = AttributeItem {
            key: parse_str("bool_key")?,
            equal_token: Default::default(),
            value: Default::default(),
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_attribute_item_complex_stream() -> Result {
        let actual: AttributeItem = parse_str("tmpl = {trait To {};}")?;
        let expected = AttributeItem {
            key: parse_str("tmpl")?,
            equal_token: Default::default(),
            value: parse_str("{trait To {};}")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    // Test extra input after a value stream is ignored
    #[test]
    #[should_panic(expected = "expected token")]
    fn parse_attribute_item_complex_stream_extra() {
        parse_str::<AttributeItem>("tmpl = {trait To {};}, key").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected identifier")]
    fn missing_key() {
        parse_str::<AttributeItem>("= true").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected `=`")]
    fn missing_equal_sign() {
        parse_str::<AttributeItem>("key  value").unwrap();
    }

    #[test]
    #[should_panic(expected = "value not supplied")]
    fn missing_value() {
        parse_str::<AttributeItem>("key = ").unwrap();
    }
}
