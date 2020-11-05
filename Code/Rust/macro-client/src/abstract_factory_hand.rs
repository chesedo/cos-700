#[allow(unused_imports)]
use macro_patterns::{abstract_factory, interpolate_traits};
use std::fmt::{Display, Formatter, Result};

use crate::gui::{
    brand_elements,
    elements::{Button, Element, Input, Window},
};

pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

pub trait AbstractGuiFactory:
    Display + Factory<dyn Button> + Factory<dyn Input> + Factory<Window>
{
}

struct BrandFactory {}
impl AbstractGuiFactory for BrandFactory {}
impl Factory<dyn Button> for BrandFactory {
    fn create(&self, name: String) -> Box<dyn Button> {
        Box::new(brand_elements::BrandButton::new(name))
    }
}
impl Factory<dyn Input> for BrandFactory {
    fn create(&self, name: String) -> Box<dyn Input> {
        Box::new(brand_elements::BrandInput::new(name))
    }
}
impl Factory<Window> for BrandFactory {
    fn create(&self, name: String) -> Box<Window> {
        Box::new(Window::new(name))
    }
}

impl Display for BrandFactory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("BrandFactory GUI creator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = BrandFactory {};
        let actual: Box<dyn Button> = factory.create(String::from("Button"));

        assert_eq!(actual.get_name(), "Button");
    }

    #[test]
    fn window_factory() {
        let factory = BrandFactory {};
        let actual: Box<Window> = factory.create(String::from("Window"));

        assert_eq!(actual.get_name(), "Window");
    }
}
