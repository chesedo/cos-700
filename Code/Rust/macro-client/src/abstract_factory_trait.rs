#[allow(unused_imports)]
use macro_patterns::{abstract_factory_trait, interpolate_traits};
use std::fmt::{Display, Formatter, Result};

use crate::gui::{
    elements::{Element, IButton, IInput, Window},
    kde::{Input, KdeButton},
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

#[abstract_factory_trait(Factory, dyn IButton, dyn IInput, Window)]
pub trait Gui: Display {}

struct KDE {}

impl Gui for KDE {}

impl Display for KDE {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("KDE GUI creator")
    }
}

#[interpolate_traits(
    IButton => KdeButton,
    IInput => Input,
)]
impl Factory<dyn TRAIT> for KDE {
    fn create(&self, name: String) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new(name))
    }
}

#[interpolate_traits(Window => Window)]
impl Factory<TRAIT> for KDE {
    fn create(&self, name: String) -> Box<TRAIT> {
        Box::new(CONCRETE::new(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = KDE {};
        let actual: Box<dyn IButton> = factory.create(String::from("Button"));

        assert_eq!(actual.name(), "Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("Window"));

        assert_eq!(actual.name(), "Window");
    }
}
