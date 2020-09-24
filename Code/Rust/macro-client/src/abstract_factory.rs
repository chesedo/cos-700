#[allow(unused_imports)]
use macro_patterns::{abstract_factory, concrete_factory};

use crate::gui::{
    elements::{Element, IButton, Window},
    kde::KdeButton,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

abstract_factory!(pub Gui, Factory, dyn IButton, Window);

struct KDE {}

impl Gui for KDE {}

concrete_factory!(
    (IButton => KdeButton),
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
