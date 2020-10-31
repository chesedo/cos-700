mod abstract_factory;

extern crate proc_macro;

use macro_lib::token_stream_utils::Interpolate;
use macro_lib::trait_specifier::TraitSpecifier;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemTrait, Token};

use abstract_factory::{abstract_factory_attribute, AbstractFactoryAttribute};

#[proc_macro_attribute]
pub fn abstract_factory_trait(tokens: TokenStream, trait_expr: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(trait_expr as ItemTrait);
    let attributes = parse_macro_input!(tokens as AbstractFactoryAttribute);

    abstract_factory_attribute(&mut input, &attributes).into()
}

#[proc_macro_attribute]
pub fn interpolate_traits(tokens: TokenStream, concrete_impl: TokenStream) -> TokenStream {
    let attributes =
        parse_macro_input!(tokens with Punctuated::<TraitSpecifier, Token![,]>::parse_terminated);

    attributes.interpolate(concrete_impl.into()).into()
}
