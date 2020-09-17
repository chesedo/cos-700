use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::collections::HashMap;
use std::hash::Hash;

/// Tokens used as interpolation markers in `quote`
#[derive(Hash, PartialEq, Eq)]
pub struct Interpolation<'a>(&'a str);

impl<'a> ToTokens for Interpolation<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('#', Spacing::Alone));
        tokens.append(Ident::new(self.0, Span::call_site()));
    }
}

/// Interpolation marked used for trait type placeholders
pub const TRAIT: Interpolation = Interpolation("TRAIT");

/// Interpolation marked used for concrete type placeholders
pub const CONCRETE: Interpolation = Interpolation("CONCRETE");

/// Trait for tokens that can replace interpolation markers
pub trait Interpolate {
    /// Take a token stream and replace interpolation markers with their actual values in a new stream
    fn interpolate(&self, stream: TokenStream) -> TokenStream;
}

/// Replace the interpolation markers in a token stream with a specific text
pub fn interpolate(
    stream: TokenStream,
    replacements: &HashMap<Interpolation, &dyn ToTokens>,
) -> TokenStream {
    let mut new = TokenStream::new();

    let mut stream = stream.into_iter().peekable();

    while let Some(token) = stream.next() {
        match token {
            TokenTree::Ident(ident) => new.append(ident),
            TokenTree::Literal(literal) => new.append(literal),
            TokenTree::Group(group) => {
                let mut new_group =
                    Group::new(group.delimiter(), interpolate(group.stream(), replacements));
                new_group.set_span(group.span());
                new.append(new_group);
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() == '#' {
                    if let Some(TokenTree::Ident(ident)) = stream.peek() {
                        if let Some(value) = replacements.get(&Interpolation(&ident.to_string())) {
                            value.to_tokens(&mut new);
                            stream.next();
                            continue;
                        }
                    }
                };

                // Preserve
                new.append(punct);
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use macro_test_helpers::reformat;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::{parse_str, Type};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn complete_replacements() -> Result {
        let input = quote! {
            const _: #TRAIT = #CONCRETE{};
        };

        let expected = quote! {
            const _: abstract_type = concrete{};
        };

        let mut r: HashMap<Interpolation, &dyn ToTokens> = HashMap::new();
        let a: Type = parse_str("abstract_type")?;
        let c: Type = parse_str("concrete")?;

        r.insert(TRAIT, &a);
        r.insert(CONCRETE, &c);

        assert_eq!(reformat(&interpolate(input, &r)), reformat(&expected));

        Ok(())
    }

    /// Partial replacements should preverse the uninterpolated identifiers
    #[test]
    fn partial_replacements() -> Result {
        let input: TokenStream = parse_str("let a: #TRAIT = #OTHER;")?;
        let expected: TokenStream = parse_str("let a: Display = #OTHER;")?;

        let mut r: HashMap<Interpolation, &dyn ToTokens> = HashMap::new();
        let t: Type = parse_str("Display")?;
        r.insert(TRAIT, &t);

        assert_eq!(
            format!("{}", interpolate(input, &r)),
            format!("{}", expected)
        );

        Ok(())
    }
}
