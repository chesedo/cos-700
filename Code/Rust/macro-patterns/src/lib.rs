mod abstract_factory;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

use abstract_factory::{
    abstract_factory_attribute, abstract_factory_function, AbstractFactoryAttribute,
    AbstractFactoryFunction,
};

#[proc_macro]
pub fn abstract_factory(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as AbstractFactoryFunction);

    abstract_factory_function(&input).into()
}

#[proc_macro_attribute]
pub fn abstract_factory_trait(tokens: TokenStream, trait_expr: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(trait_expr as ItemTrait);
    let attributes = parse_macro_input!(tokens as AbstractFactoryAttribute);

    abstract_factory_attribute(&mut input, &attributes).into()
}
