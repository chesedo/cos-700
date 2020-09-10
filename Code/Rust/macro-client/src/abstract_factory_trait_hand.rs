#[allow(unused_imports)]
use macro_patterns::abstract_factory_trait;
use std::fmt::{Display, Formatter, Result};

use crate::gui::{
    elements::{Element, IButton, Window},
    kde::KdeButton,
};

trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

trait Gui: Display + Factory<dyn IButton> + Factory<Window> {}

struct KDE {}

impl Gui for KDE {}

impl Display for KDE {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("KDE GUI creator")
    }
}

impl Factory<dyn IButton> for KDE {
    fn create(&self) -> Box<dyn IButton> {
        Box::new(KdeButton::new(String::from("KDE Button")))
    }
}
impl Factory<Window> for KDE {
    fn create(&self) -> Box<Window> {
        Box::new(Window::new(String::from("Window")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = KDE {};
        let actual: Box<dyn IButton> = factory.create();

        assert_eq!(actual.name(), "KDE Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create();

        assert_eq!(actual.name(), "Window");
    }
}
