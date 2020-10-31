#[allow(unused_imports)]
use macro_patterns::{abstract_factory, interpolate_traits};
use std::fmt::{Display, Formatter, Result};

use crate::gui::{
    elements::{Button, Element, Input, Window},
    kde,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

pub trait Gui: Display + Factory<dyn Button> + Factory<dyn Input> + Factory<Window> {}

struct KDE {}

impl Gui for KDE {}

impl Display for KDE {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("KDE GUI creator")
    }
}

impl Factory<dyn Button> for KDE {
    fn create(&self, name: String) -> Box<dyn Button> {
        Box::new(kde::KdeButton::new(name))
    }
}
impl Factory<dyn Input> for KDE {
    fn create(&self, name: String) -> Box<dyn Input> {
        Box::new(kde::Input::new(name))
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
        let actual: Box<dyn Button> = factory.create(String::from("Button"));

        assert_eq!(actual.get_name(), "Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("Window"));

        assert_eq!(actual.get_name(), "Window");
    }
}
