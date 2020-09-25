#[allow(unused_imports)]
use crate::{abstract_factory_r, traits_expansion};

use crate::gui::{
    elements::{Button, Element, Window},
    kde::KdeButton,
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

pub trait Gui
where
    Self: Factory<dyn Button>,
    Self: Factory<Window>,
{
}

struct KDE {}

impl Gui for KDE {}

impl Factory<dyn Button> for KDE {
    fn create(&self, name: String) -> Box<dyn Button> {
        Box::new(KdeButton::new(name))
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
        let actual: Box<dyn Button> = factory.create(String::from("Some button"));

        assert_eq!(actual.get_name(), "Some button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("First window"));

        assert_eq!(actual.get_name(), "First window");
    }
}
