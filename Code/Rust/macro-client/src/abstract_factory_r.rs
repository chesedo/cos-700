#[allow(unused_imports)]
use crate::{abstract_factory_r, concrete_factory_r};

trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}
pub trait Element {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
}

mod button {
    use super::Element;

    pub trait IButton: Element {
        fn click(&self);
    }

    pub struct KdeButton {
        name: String,
    }

    impl Element for KdeButton {
        fn new(name: String) -> Self {
            KdeButton { name }
        }
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl IButton for KdeButton {
        fn click(&self) {
            unimplemented!()
        }
    }
}

mod window {
    use super::Element;

    pub struct Window {
        name: String,
    }

    impl Element for Window {
        fn new(name: String) -> Self {
            Window { name }
        }
        fn name(&self) -> &str {
            &self.name
        }
    }
}

macro_rules! factory {
    ($trait:ty, $concrete:ident) => {
        impl Factory<$trait> for KDE {
            fn create(&self, name: String) -> Box<$trait> {
                Box::new($concrete::new(name))
            }
        }
    };
}

use button::{IButton, KdeButton};
use window::Window;

abstract_factory_r!(Gui, Factory, dyn IButton, Window);

struct KDE {}

impl Gui for KDE {}

concrete_factory_r!(dyn IButton => KdeButton, Window => Window);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = KDE {};
        let actual: Box<dyn IButton> = factory.create(String::from("Close Button"));

        assert_eq!(actual.name(), "Close Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("Main Window"));

        assert_eq!(actual.name(), "Main Window");
    }
}
