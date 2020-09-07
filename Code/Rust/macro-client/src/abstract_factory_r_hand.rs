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

use button::{IButton, KdeButton};
use window::Window;

trait Gui
where
    Self: Factory<dyn IButton>,
    Self: Factory<Window>,
{
}

struct KDE {}

impl Gui for KDE {}

impl Factory<dyn IButton> for KDE {
    fn create(&self, name: String) -> Box<dyn IButton> {
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
        let actual: Box<dyn IButton> = factory.create(String::from("Some button"));

        assert_eq!(actual.name(), "Some button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Box<Window> = factory.create(String::from("First window"));

        assert_eq!(actual.name(), "First window");
    }
}
