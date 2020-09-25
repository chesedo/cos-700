#[allow(unused_imports)]
use crate::{abstract_factory_r, traits_expansion};

use crate::gui::{
    elements::{Button, Element, Window},
    kde::KdeButton,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

abstract_factory_r!(pub Gui, Factory, dyn Button, Window);

struct KDE {}

impl Gui for KDE {}

macro_rules! trait_expand {
    ($trait:ty, $concrete:ident) => {
        impl Factory<$trait> for KDE {
            fn create(&self, name: String) -> Box<$trait> {
                Box::new($concrete::new(name))
            }
        }
    };
}

traits_expansion!(dyn Button => KdeButton, Window => Window);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = KDE {};
        let actual: Box<dyn Button> = factory.create(String::from("Close Button"));

        assert_eq!(actual.get_name(), "Close Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("Main Window"));

        assert_eq!(actual.get_name(), "Main Window");
    }
}
