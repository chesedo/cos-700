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

#[|\functionh{abstract\_factory}|(|\inputh{Factory}|, |\inputbh{dyn Button, dyn Input, Window}|)]
|\contexth{pub trait AbstractGuiFactory: Display \{\}}|

struct BrandFactory {}
impl AbstractGuiFactory for BrandFactory {}

#[|\functionbh{interpolate\_traits}|(
    Button => brand_elements::BrandButton,
    Input => brand_elements::BrandInput,
)]
impl Factory<dyn TRAIT> for BrandFactory {
    fn create(&self, name: String) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new(name))
    }
}

#[|\functionbh{interpolate\_traits}|(Window => Window)]
impl Factory<TRAIT> for BrandFactory {
    fn create(&self, name: String) -> Box<TRAIT> {
        Box::new(CONCRETE::new(name))
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
