use macro_lib::trait_specifier::TraitSpecifier;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_quote, token, ItemTrait, Token, Type, TypeParamBound, Visibility};

/// Holds tokens for AbstractFactory functional macro inputs
/// Expects an input in the following format
/// ```text
/// [visibility] trait_identifier, some_abstract_factory_trait, type_1, type_2, ... , type_n
/// ```
#[derive(Eq, PartialEq, Debug)]
pub struct AbstractFactoryFunction {
    vis: Visibility,
    trait_ident: Type,
    first_sep: Token![,],
    factory_trait: Type,
    second_sep: Token![,],
    types: Punctuated<Type, Token![,]>,
}

impl Parse for AbstractFactoryFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AbstractFactoryFunction {
            vis: input.parse()?,
            trait_ident: input.parse()?,
            first_sep: input.parse()?,
            factory_trait: input.parse()?,
            second_sep: input.parse()?,
            types: input.parse_terminated(Type::parse)?,
        })
    }
}

/// Holds tokens for ConcreteFactory functional macro inputs
/// Expects an input in the following format
/// ```text
/// (trait1 => concrete1, trait2 => concrete2, ...), stream_of_tokens_to_replace
/// ```
///
/// Each `TRAIT` and `CONCRETE` token it the passed in stream will be replaced
/// with a `trait => concrete` set.
#[derive(Debug)]
pub struct ConcreteFactoryFunction {
    paren_token: token::Paren,
    pub traits: Punctuated<TraitSpecifier, Token![,]>,
    comma: Token![,],
    pub implementation: TokenStream,
}

impl Parse for ConcreteFactoryFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let traits_content;

        Ok(ConcreteFactoryFunction {
            paren_token: parenthesized!(traits_content in input),
            traits: traits_content.parse_terminated(TraitSpecifier::parse)?,
            comma: input.parse()?,
            implementation: input.parse()?,
        })
    }
}

/// Holds the tokens for the attributes passed to an AbstractFactory attribute macro
/// Expects input in the following format
/// ```text
/// some_abstract_factory_trait, type_1, type_2, ... , type_n
/// ```
#[derive(Eq, PartialEq, Debug)]
pub struct AbstractFactoryAttribute {
    factory_trait: Type,
    sep: Token![,],
    types: Punctuated<Type, Token![,]>,
}

impl Parse for AbstractFactoryAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AbstractFactoryAttribute {
            factory_trait: input.parse()?,
            sep: input.parse()?,
            types: input.parse_terminated(Type::parse)?,
        })
    }
}

/// Expands an [AbstractFactoryFunction](struct.AbstractFactoryFunction.html) to a TokenStream
pub fn abstract_factory_function(input: &AbstractFactoryFunction) -> TokenStream {
    let vis = &input.vis;
    let abstract_name = &input.trait_ident;
    let factory_name = &input.factory_trait;

    let bounds = {
        let types = input.types.iter();

        quote! {
            #(#factory_name<#types>)+*
        }
    };

    quote! {
        #vis trait #abstract_name: #bounds {}
    }
}

/// Expands a trait definition together with an [AbstractFactoryAttribute](struct.AbstractFactoryAttribute.html) to a TokenStream
pub fn abstract_factory_attribute(
    input_trait: &mut ItemTrait,
    attributes: &AbstractFactoryAttribute,
) -> TokenStream {
    let bounds: Punctuated<TypeParamBound, Token![+]> = {
        let types = attributes.types.iter();
        let factory_name = &attributes.factory_trait;

        parse_quote! {
            #(#factory_name<#types>)+*
        }
    };

    // Add extra bounds
    input_trait.supertraits.extend(bounds);

    quote! {
        #input_trait
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macro_test_helpers::reformat;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    mod abstract_factory_function {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn parse() -> Result {
            let actual: AbstractFactoryFunction = parse_str("Gui, Factory, u32, i64")?;
            let mut expected_types = Punctuated::new();

            expected_types.push(parse_str("u32")?);
            expected_types.push(parse_str("i64")?);

            assert_eq!(
                actual,
                AbstractFactoryFunction {
                    vis: parse_str("")?,
                    trait_ident: parse_str("Gui")?,
                    first_sep: Default::default(),
                    factory_trait: parse_str("Factory")?,
                    second_sep: Default::default(),
                    types: expected_types,
                }
            );

            Ok(())
        }

        #[test]
        #[should_panic(expected = "expected `,`")]
        fn missing_types() {
            parse_str::<AbstractFactoryFunction>("Trait, Factory").unwrap();
        }

        #[test]
        fn expand() -> Result {
            let mut input_types = Punctuated::new();

            input_types.push(parse_str("u32")?);
            input_types.push(parse_str("i64")?);

            let actual = abstract_factory_function(&AbstractFactoryFunction {
                vis: parse_str("pub")?,
                trait_ident: parse_str("Gui")?,
                first_sep: Default::default(),
                factory_trait: parse_str("Factory")?,
                second_sep: Default::default(),
                types: input_types,
            });
            assert_eq!(
                reformat(&actual),
                "pub trait Gui: Factory<u32> + Factory<i64> {}\n"
            );

            Ok(())
        }
    }

    mod concrete_factory_function {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn parse() -> Result {
            let actual: ConcreteFactoryFunction =
                parse_str("(IButton => Button, IInput => Input), const _: TRAIT = CONCRETE{};")?;
            let mut expected_traits = Punctuated::new();

            expected_traits.push(parse_str("IButton => Button")?);
            expected_traits.push(parse_str("IInput => Input")?);

            assert_eq!(actual.traits, expected_traits);
            assert_eq!(
                reformat(&actual.implementation),
                "const _: TRAIT = CONCRETE {};\n"
            );

            Ok(())
        }
    }

    mod abstract_factory_attribute {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn parse() -> Result {
            let actual: AbstractFactoryAttribute = parse_str("Factory, u32, i64")?;
            let mut expected_types = Punctuated::new();

            expected_types.push(parse_str("u32")?);
            expected_types.push(parse_str("i64")?);

            assert_eq!(
                actual,
                AbstractFactoryAttribute {
                    factory_trait: parse_str("Factory")?,
                    sep: Default::default(),
                    types: expected_types,
                }
            );

            Ok(())
        }

        #[test]
        #[should_panic(expected = "expected `,`")]
        fn missing_types() {
            parse_str::<AbstractFactoryAttribute>("Factory").unwrap();
        }

        #[test]
        fn expand() -> Result {
            let mut t = parse_str::<ItemTrait>("pub trait Abstraction<T>: Display + Extend<T> {}")?;
            let mut input_types = Punctuated::new();

            input_types.push(parse_str("u32")?);
            input_types.push(parse_str("i64")?);

            let actual = abstract_factory_attribute(
                &mut t,
                &AbstractFactoryAttribute {
                    factory_trait: parse_str("Factory")?,
                    sep: Default::default(),
                    types: input_types,
                },
            );

            assert_eq!(
                reformat(&actual),
                "pub trait Abstraction<T>: Display + Extend<T> + Factory<u32> + Factory<i64> {}\n"
            );

            Ok(())
        }
    }
}
