#[allow(unused_imports)]
use macro_patterns::{abstract_factory, concrete_factory};

use crate::gui::{
    elements::{Button, Element, Window},
    kde::KdeButton,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

abstract_factory!(pub Gui, Factory, dyn Button, Window);

struct KDE {}

impl Gui for KDE {}

concrete_factory!(
    (Button => KdeButton),
    impl Factory<dyn TRAIT> for KDE {
        fn create(&self) -> Box<dyn TRAIT> {
            Box::new(CONCRETE::new(String::from("KDE Button")))
        }
    }
);

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
        let actual: Box<dyn Button> = factory.create();

        assert_eq!(actual.get_name(), "KDE Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create();

        assert_eq!(actual.get_name(), "Window");
    }
}
