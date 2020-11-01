use crate::options_attribute::OptionsAttribute;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type};

/// Holds a type that is optionally annotated with key-value options
#[derive(Eq, PartialEq, Debug)]
pub struct RichType<T = Type> {
    pub attrs: OptionsAttribute,
    pub ident: T,
}

/// Make RichType parsable from token stream
impl<T: Parse> Parse for RichType<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![#]) {
            return Ok(RichType {
                attrs: input.parse()?,
                ident: input.parse()?,
            });
        };

        Ok(RichType {
            attrs: Default::default(),
            ident: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, parse_str, TypeTraitObject};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse_rich_type() -> Result {
        let actual: RichType = parse_quote! {
            #[no_default]
            i32
        };
        let expected = RichType {
            attrs: parse_str("#[no_default]")?,
            ident: parse_str("i32")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_simple_type() -> Result {
        let actual: RichType = parse_quote! {
            Button
        };
        let expected = RichType {
            attrs: Default::default(),
            ident: parse_str("Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_trait_bounds() -> Result {
        let actual: RichType<TypeTraitObject> = parse_quote! {
            #[no_default]
            dyn Button
        };
        let expected = RichType::<TypeTraitObject> {
            attrs: parse_str("#[no_default]")?,
            ident: parse_str("dyn Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "unexpected end of input")]
    fn missing_type() {
        parse_str::<RichType>("#[no_default]").unwrap();
    }
}
