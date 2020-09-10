//! A generic token list to parse items seperated by a comma

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Token, Type, TypeParamBound, Visibility};

/// Holds a set of `T` items seperated by a comma. It will be able to parse string of the following format:
/// ```text
/// T1, T2, T3
/// ```
///
/// T can be any parsable token or set of common tokens.
#[derive(Eq, PartialEq, Debug)]
pub struct TokenList<T: Parse> {
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

impl TokenList<Type> {
    pub fn to_factory_bounds(&self, factory_name: &Type) -> TokenStream {
        let types = self.types.iter();

        quote! {
            #(#factory_name<#types>)+*
        }
    }

    pub fn to_type_param_bounds(
        &self,
        factory_name: &Type,
    ) -> Punctuated<TypeParamBound, Token![+]> {
        let types = self.types.iter();

        parse_quote! {
            #(#factory_name<#types>)+*
        }
    }

    pub fn to_abstract_factory(
        &self,
        abstract_name: &Type,
        factory_name: &Type,
    ) -> TokenStream {
        let bounds = self.to_factory_bounds(factory_name);

        quote! {
            trait #abstract_name: #bounds {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trait_specifier::TraitSpecifier;
    use syn::{parse_str, Type};

    use macro_test_helpers::reformat;

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

    #[test]
    fn to_factory_bounds() -> Result {
        let list: TokenList<Type> = parse_str("IButton, IWindow")?;
        let bounds = &list.to_factory_bounds(&parse_str("Factory")?);

        assert_eq!(
            reformat(&quote! {trait Test: #bounds {}}),
            "trait Test: Factory<IButton> + Factory<IWindow> {}\n"
        );

        Ok(())
    }

    #[test]
    fn to_type_param_bounds() -> Result {
        let list: TokenList<Type> = parse_str("IButton, IWindow")?;
        let bounds = &list.to_type_param_bounds(&parse_str("Factory")?);

        assert_eq!(
            reformat(&quote! {trait Test: #bounds {}}),
            "trait Test: Factory<IButton> + Factory<IWindow> {}\n"
        );

        Ok(())
    }

    #[test]
    fn to_abstract_factory() -> Result {
        let list: TokenList<Type> = parse_str("IButton, IWindow")?;

        assert_eq!(
            reformat(
                &list.to_abstract_factory(&parse_str("UiFactory")?, &parse_str("ElementFactory")?)
            ),
            "trait UiFactory: ElementFactory<IButton> + ElementFactory<IWindow> {}\n"
        );

        Ok(())
    }
}
