#[allow(unused_imports)]
use macro_patterns_dec::{abstract_factory, concrete_factory};

use crate::gui::{
    brand_elements::BrandButton,
    elements::{Button, Element, Window},
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

pub trait AbstractGuiFactory: Factory<dyn Button> + Factory<Window> {}

struct BrandFactory {}

impl AbstractGuiFactory for BrandFactory {}

impl Factory<dyn Button> for BrandFactory {
    fn create(&self, name: String) -> Box<dyn Button> {
        Box::new(<BrandButton>::new(name))
    }
}
impl Factory<Window> for BrandFactory {
    fn create(&self, name: String) -> Box<Window> {
        Box::new(<Window>::new(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = BrandFactory {};
        let actual: Box<dyn Button> = factory.create(String::from("Some button"));

        assert_eq!(actual.get_name(), "Some button");
    }

    #[test]
    fn window_factory() {
        let factory = BrandFactory {};
        let actual: Box<Window> = factory.create(String::from("First window"));

        assert_eq!(actual.get_name(), "First window");
    }
}
