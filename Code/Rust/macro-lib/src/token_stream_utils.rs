use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::collections::HashMap;

/// Trait for tokens that can replace interpolation markers
pub trait Interpolate {
    /// Take a token stream and replace interpolation markers with their actual values in a new stream
    fn interpolate(&self, stream: TokenStream) -> TokenStream;
}

/// Replace the interpolation markers in a token stream with a specific text
pub fn interpolate(
    stream: TokenStream,
    replacements: &HashMap<&str, &dyn ToTokens>,
) -> TokenStream {
    let mut new = TokenStream::new();

    for token in stream.into_iter() {
        match token {
            TokenTree::Ident(ident) => {
                let ident_str: &str = &ident.to_string();

                if let Some(value) = replacements.get(ident_str) {
                    value.to_tokens(&mut new);
                    continue;
                }

                new.append(ident);
            }
            TokenTree::Literal(literal) => new.append(literal),
            TokenTree::Group(group) => {
                let mut new_group =
                    Group::new(group.delimiter(), interpolate(group.stream(), replacements));
                new_group.set_span(group.span());
                new.append(new_group);
            }
            TokenTree::Punct(punct) => {
                new.append(punct);
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::{parse_str, Ident, Type};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn complete_replacements() -> Result {
        let input = quote! {
            let VAR: TRAIT = if true {
                CONCRETE{}
            } else {
                Alternative{}
            }
        };

        let expected = quote! {
            let var: abstract_type = if true {
                concrete{}
            } else {
                Alternative{}
            }
        };

        let mut r: HashMap<&str, &dyn ToTokens> = HashMap::new();
        let v: Ident = parse_str("var")?;
        let a: Type = parse_str("abstract_type")?;
        let c: Type = parse_str("concrete")?;

        r.insert("VAR", &v);
        r.insert("TRAIT", &a);
        r.insert("CONCRETE", &c);

        assert_eq!(
            format!("{}", &interpolate(input, &r)),
            format!("{}", expected)
        );

        Ok(())
    }

    /// Partial replacements should preverse the uninterpolated identifiers
    #[test]
    fn partial_replacements() -> Result {
        let input: TokenStream = parse_str("let a: TRAIT = OTHER;")?;
        let expected: TokenStream = parse_str("let a: Display = OTHER;")?;

        let mut r: HashMap<&str, &dyn ToTokens> = HashMap::new();
        let t: Type = parse_str("Display")?;
        r.insert("TRAIT", &t);

        assert_eq!(
            format!("{}", interpolate(input, &r)),
            format!("{}", expected)
        );

        Ok(())
    }
}
