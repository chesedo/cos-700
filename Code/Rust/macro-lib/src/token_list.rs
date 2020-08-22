//! A generic token list to parse items seperated by a comma

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Holds a set of `T` items seperated by a comma. It will be able to parse string of the following format:
/// ```
/// T1, T2, T3
/// ```
///
/// T can be any parsable token or set of common tokens.
#[derive(Eq, PartialEq, Debug)]
struct TokenList<T: Parse> {
    types: Punctuated<T, Comma>,
}

/// Make TokenList parsable from a token stream
/// Was initially based on https://docs.rs/syn/1.0.31/src/syn/ty.rs.html#804-830, but that code does not allow a single item
impl<T: Parse> Parse for TokenList<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TokenList {
            types: Punctuated::parse_terminated(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trait_specifier::TraitSpecifier;
    use syn::{parse_str, Type};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parsing_empty_input() -> Result {
        let actual: TokenList<Type> = parse_str("")?;

        assert_eq!(
            actual,
            TokenList {
                types: Punctuated::new()
            }
        );

        Ok(())
    }

    #[test]
    fn type_parsing() -> Result {
        let actual: TokenList<Type> = parse_str("u32, i64")?;

        let mut expected_types: Punctuated<Type, Comma> = Punctuated::new();
        expected_types.push(parse_str("u32")?);
        expected_types.push(parse_str("i64")?);

        assert_eq!(
            actual,
            TokenList {
                types: expected_types
            }
        );

        Ok(())
    }

    #[test]
    fn type_parsing_one_item() -> Result {
        let actual: TokenList<Type> = parse_str("&str")?;

        let mut expected_types: Punctuated<Type, Comma> = Punctuated::new();
        expected_types.push(parse_str("&str")?);

        assert_eq!(
            actual,
            TokenList {
                types: expected_types
            }
        );

        Ok(())
    }

    #[test]
    fn trait_specifier_parsing() -> Result {
        let actual: TokenList<TraitSpecifier> =
            parse_str("IButton => BigButton, IWindow => MinimalWindow")?;

        let mut expected_types: Punctuated<TraitSpecifier, Comma> = Punctuated::new();
        expected_types.push(parse_str("IButton => BigButton")?);
        expected_types.push(parse_str("IWindow => MinimalWindow")?);

        assert_eq!(
            actual,
            TokenList {
                types: expected_types
            }
        );

        Ok(())
    }

}
