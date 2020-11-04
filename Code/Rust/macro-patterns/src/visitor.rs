use macro_lib::{extensions::ToLowercase, AnnotatedType, KeyValue, SimpleType};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

#[derive(Eq, PartialEq, Debug)]
pub struct VisitorFunction {
    types: Punctuated<AnnotatedType<SimpleType>, Token![,]>,
}

impl Parse for VisitorFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(VisitorFunction {
            types: input.parse_terminated(AnnotatedType::parse)?,
        })
    }
}

impl VisitorFunction {
    pub fn expand(&self) -> TokenStream {
        let mut trait_functions: Vec<TokenStream> = Vec::new();
        let mut helpers: Vec<TokenStream> = Vec::new();
        let mut visitables: Vec<TokenStream> = Vec::new();

        for t in self.types.iter() {
            let elem_name = t.inner_type.ident.to_lowercase();
            let elem_type = &t.inner_type;

            let fn_name = format_ident!("visit_{}", elem_name);

            let options = Options::new(&t.attrs.options);

            let fn_def = if options.no_default {
                quote! {
                    fn #fn_name(&mut self, #elem_name: &#elem_type);
                }
            } else {
                quote! {
                    fn #fn_name(&mut self, #elem_name: &#elem_type) {
                        #fn_name(self, #elem_name)
                    }
                }
            };

            if options.has_helper {
                if let Some(inner) = options.helper_tmpl {
                    helpers.push(quote! {
                        pub fn #fn_name<V>(visitor: &mut V, #elem_name: &#elem_type)
                        where
                            V: Visitor + ?Sized,
                        {
                            #inner
                        }
                    });
                } else {
                    let unused_elem_name = format_ident!("_{}", elem_name);
                    helpers.push(quote! {
                        pub fn #fn_name<V>(_visitor: &mut V, #unused_elem_name: &#elem_type)
                        where
                            V: Visitor + ?Sized,
                        {
                        }
                    });
                }
            };

            trait_functions.push(fn_def);
            visitables.push(quote! {
                impl Visitable for #elem_type {
                    fn apply(&self, visitor: &mut dyn Visitor) {
                        visitor.#fn_name(self);
                    }
                }
            });
        }

        quote! {
            pub trait Visitor {
                #(#trait_functions)*
            }

            #(#helpers)*

            trait Visitable {
                fn apply(&self, visitor: &mut dyn Visitor);
            }
            #(#visitables)*
        }
    }
}

struct Options {
    no_default: bool,
    has_helper: bool,
    helper_tmpl: Option<TokenStream>,
}

impl Options {
    fn new(options: &Punctuated<KeyValue, Token![,]>) -> Self {
        let mut no_default = false;
        let mut has_helper = true;
        let mut helper_tmpl = None;

        for option in options.iter() {
            if option.key == Ident::new("no_default", Span::call_site()) {
                no_default = true;
                continue;
            }

            if option.key == Ident::new("helper_tmpl", Span::call_site()) {
                match option.value.clone().into_iter().next().unwrap() {
                    TokenTree::Ident(ident) if ident == Ident::new("false", Span::call_site()) => {
                        has_helper = false;
                    }
                    TokenTree::Group(group) => {
                        helper_tmpl = Some(group.stream());
                    }
                    _ => continue,
                }
            }
        }

        Options {
            no_default,
            has_helper,
            helper_tmpl,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macro_test_helpers::reformat;
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, parse_str};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() {
        let actual: VisitorFunction = parse_quote! {
            #[no_default]
            dyn Button
        };

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_quote! {#[no_default] dyn Button});

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_just_types() -> Result {
        let actual: VisitorFunction = parse_str("Button, dyn Text, Window")?;

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_str("Button")?);
        expected.types.push(parse_str("dyn Text")?);
        expected.types.push(parse_str("Window")?);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_mixed() -> Result {
        let actual: VisitorFunction = parse_quote! {
            Button,

            #[tmpl = {trait T {};}]
            Text,

            dyn Window
        };

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_str("Button")?);
        expected.types.push(parse_quote! {
            #[tmpl = {trait T {};}]
            Text
        });
        expected.types.push(parse_str("dyn Window")?);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn expand() -> Result {
        let mut input = VisitorFunction {
            types: Punctuated::new(),
        };

        input.types.push(parse_quote! {
            #[helper_tmpl = false]
            Button
        });
        input.types.push(parse_quote! {
            #[no_default]
            dyn Text
        });
        input.types.push(parse_quote! {
           #[helper_tmpl = {
               visitor.visit_button(window.button);
           }]
           Window
        });

        let actual = input.expand();
        let expected = quote! {
            pub trait Visitor{
                fn visit_button(&mut self, button: &Button) {
                    visit_button(self, button)
                }
                fn visit_text(&mut self, text: &dyn Text);
                fn visit_window(&mut self, window: &Window) {
                    visit_window(self, window)
                }
            }

            pub fn visit_text<V>(_visitor: &mut V, _text: &dyn Text)
            where
                V: Visitor + ?Sized,
            {
            }

            pub fn visit_window<V>(visitor: &mut V, window: &Window)
            where
                V: Visitor + ?Sized,
            {
               visitor.visit_button(window.button);
            }

            trait Visitable {
                fn apply(&self, visitor: &mut dyn Visitor);
            }
            impl Visitable for Button {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.visit_button(self);
                }
            }
            impl Visitable for dyn Text {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.visit_text(self);
                }
            }
            impl Visitable for Window {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.visit_window(self);
                }
            }
        };

        assert_eq!(
            reformat(&actual).lines().collect::<Vec<_>>(),
            reformat(&expected).lines().collect::<Vec<_>>()
        );

        Ok(())
    }
}
