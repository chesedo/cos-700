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

pub trait Gui: Display + Factory<dyn IButton> + Factory<dyn IInput> + Factory<Window> {}

struct KDE {}

impl Gui for KDE {}

impl Display for KDE {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("KDE GUI creator")
    }
}

impl Factory<dyn IButton> for KDE {
    fn create(&self, name: String) -> Box<dyn IButton> {
        Box::new(KdeButton::new(name))
    }
}
impl Factory<dyn IInput> for KDE {
    fn create(&self, name: String) -> Box<dyn IInput> {
        Box::new(Input::new(name))
    }
}
impl Factory<Window> for KDE {
    fn create(&self, name: String) -> Box<Window> {
        Box::new(Window::new(name))
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
