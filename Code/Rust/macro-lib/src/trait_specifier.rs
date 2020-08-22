use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type};

/// Type that holds an abstract type and how it will map to a concrete type.
/// An acceptable stream will have the following form:
/// ```
/// trait => concrete
/// ```
#[derive(Eq, PartialEq, Debug)]
pub struct TraitSpecifier {
    abstract_trait: Type,
    arrow_token: Token![=>],
    concrete: Type,
}

/// Make TraitSpecifier parsable from a token stream
impl Parse for TraitSpecifier {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TraitSpecifier {
            abstract_trait: input.parse()?,
            arrow_token: input.parse()?,
            concrete: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse_trait_specifier() -> Result {
        let actual: TraitSpecifier = parse_str("abstract_trait => concrete")?;
        let expected = TraitSpecifier {
            abstract_trait: parse_str("abstract_trait")?,
            arrow_token: Default::default(),
            concrete: parse_str("concrete")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "expected one of")]
    fn missing_trait() {
        parse_str::<TraitSpecifier>("=> concrete").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected `=>`")]
    fn missing_arrow_joiner() {
        parse_str::<TraitSpecifier>("IButton -> RoundButton").unwrap();
    }

    #[test]
    #[should_panic(expected = "unexpected end of input")]
    fn missing_concrete() {
        parse_str::<TraitSpecifier>("abstract_trait => ").unwrap();
    }
}
