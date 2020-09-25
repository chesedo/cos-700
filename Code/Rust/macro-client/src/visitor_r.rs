#[allow(unused_imports)]
use crate::gui::elements::{Child, Element, IButton, IInput, Window};
use crate::traits_expansion;
use paste::paste;
use std::fmt;
use std::ops::Deref;

// Set expansion for the trait functions
macro_rules! trait_expand {
    ($trait:ty, $name:ident) => {
        paste! {
            fn [<visit_ $name>](&mut self, $name: $trait) {
                [<visit_ $name>](self, $name)
            }
        }
    };
}
pub trait Visitor {
    traits_expansion!(
        &dyn Element => element,
        &dyn IButton => button,
        &dyn IInput => input,
        &Window => window,
    );
}

// Set the expansion for the empty helper functions
macro_rules! trait_expand {
    ($trait:ty, $name:ident) => {
        paste! {
            pub fn [<visit_ $name>]<V>(_visitor: &mut V, [<_ $name>]: $trait)
            where
                V: Visitor + ?Sized,
            { }
        }
    };
}
traits_expansion!(
    &dyn Element => element,
    &dyn IButton => button,
    &dyn IInput => input,
);

pub fn visit_window<V>(visitor: &mut V, window: &Window)
where
    V: Visitor + ?Sized,
{
    window.get_children().iter().for_each(|child| {
        match child {
            Child::Button(button) => {
                visitor.visit_button(button.read().expect("Button is no longer readable").deref())
            }
            Child::Input(input) => {
                visitor.visit_input(input.read().expect("Input is no longer readable").deref())
            }
        };
    });
}

trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}

// Set the expansion for 'reflection` trait helpers
macro_rules! trait_expand {
    ($trait:ty, $name:ident) => {
        paste! {
            impl Visitable for $trait {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.[<visit_ $name>](self);
                }
            }
        }
    };
}
traits_expansion!(
    Window => window,
    dyn Element => element,
    dyn IButton => button,
    dyn IInput => input,
);

struct VisitorName {
    names: Vec<String>,
}

impl VisitorName {
    pub fn new() -> Self {
        VisitorName { names: Vec::new() }
    }
}

impl fmt::Display for VisitorName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.names.join(", "))
    }
}

impl Visitor for VisitorName {
    fn visit_button(&mut self, button: &dyn IButton) {
        self.names.push(button.get_name().to_string());
    }

    fn visit_input(&mut self, input: &dyn IInput) {
        self.names
            .push(format!("{} ({})", input.get_name(), input.get_input()));
    }

    fn visit_window(&mut self, window: &Window) {
        self.names.push(window.get_name().to_string());

        visit_window(self, window);
    }
}
