#[allow(unused_imports)]
use macro_patterns_dec::{abstract_factory, concrete_factory};

use crate::gui::{
    brand_elements::BrandButton,
    elements::{Button, Element, Window},
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

abstract_factory!(
    pub trait AbstractGuiFactory: Factory<T> {
     dyn Button,
     Window,
    }
);

struct BrandFactory {}

impl AbstractGuiFactory for BrandFactory {}

macro_rules! create_named {
    ($concrete:ty: $trait:ty) => {
        fn create(&self, name: String) -> Box<$trait> {
            Box::new(<$concrete>::new(name))
        }
    };
}

concrete_factory!(
    create_named(
        impl Factory<T> for BrandFactory {
            BrandButton: dyn Button,
            Window: Window,
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = BrandFactory {};
        let actual: Box<dyn Button> = factory.create(String::from("Close Button"));

        assert_eq!(actual.get_name(), "Close Button");
    }

    #[test]
    fn window_factory() {
        let factory = BrandFactory {};
        let actual: Box<Window> = factory.create(String::from("Main Window"));

        assert_eq!(actual.get_name(), "Main Window");
    }
}
