#[allow(unused_imports)]
use macro_patterns::{abstract_factory, concrete_factory};

use crate::gui::{
    elements::{Element, IButton, Window},
    kde::KdeButton,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

pub trait Gui: Factory<dyn IButton> + Factory<Window> {}

struct KDE {}

impl Gui for KDE {}

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
        let mut actual: Box<dyn IButton> = factory.create();

        actual.set_text(String::from("Close document"));

        assert_eq!(actual.get_name(), "KDE Button");
        assert_eq!(actual.get_text(), "Close document");
    }

    #[test]
    fn window_factory() {
        let factory: &dyn Factory<Window> = &KDE {};
        let actual = factory.create();

        assert_eq!(actual.get_name(), "Window");
    }
}
