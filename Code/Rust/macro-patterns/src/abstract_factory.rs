use macro_lib::token_list::TokenList;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type};

/// Holds tokens for AbstractFactory functional macro inputs
/// Expects an input in the following format
/// ```text
/// trait_identifier, some_abstract_factory_trait, type_1, type_2, ... , type_n
/// ```
#[derive(Eq, PartialEq, Debug)]
pub struct AbstractFactoryFunction {
    trait_ident: Type,
    first_sep: Token![,],
    factory_trait: Type,
    second_sep: Token![,],
    types: TokenList<Type>,
}

impl Parse for AbstractFactoryFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AbstractFactoryFunction {
            trait_ident: input.parse()?,
            first_sep: input.parse()?,
            factory_trait: input.parse()?,
            second_sep: input.parse()?,
            types: input.parse()?,
        })
    }
}

/// Expands an [AbstractFactoryFunction](struct.AbstractFactoryFunction.html) to a TokenStream
pub fn abstract_factory_function(input: &AbstractFactoryFunction) -> TokenStream {
    input
        .types
        .to_abstract_factory(&input.trait_ident, &input.factory_trait)
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

            assert_eq!(
                actual,
                AbstractFactoryFunction {
                    trait_ident: parse_str("Gui")?,
                    first_sep: Default::default(),
                    factory_trait: parse_str("Factory")?,
                    second_sep: Default::default(),
                    types: parse_str("u32, i64")?,
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
            let actual = abstract_factory_function(&AbstractFactoryFunction {
                trait_ident: parse_str("Gui")?,
                first_sep: Default::default(),
                factory_trait: parse_str("Factory")?,
                second_sep: Default::default(),
                types: parse_str("u32, i64")?,
            });
            assert_eq!(
                reformat(&actual),
                "trait Gui: Factory<u32> + Factory<i64> {}\n"
            );

            Ok(())
        }
    }
}
