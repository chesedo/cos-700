#[allow(unused_imports)]
use macro_patterns::abstract_factory;

pub trait Element {
    fn name(&self) -> &str;
}

mod button {
    use super::Element;

    pub trait IButton: Element {
        fn click(&self);
    }

    pub struct KdeButton {}

    impl Element for KdeButton {
        fn name(&self) -> &str {
            "KDE Button"
        }
    }

    impl IButton for KdeButton {
        fn click(&self) {
            unimplemented!()
        }
    }
}

mod window {
    use super::*;

    pub struct Window {}

    impl Element for Window {
        fn name(&self) -> &str {
            "Window"
        }
    }
}

use button::{IButton, KdeButton};
use window::Window;

trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

abstract_factory!(Gui, Factory, dyn IButton, Window);

struct KDE {}

impl Gui for KDE {}

impl Factory<dyn IButton> for KDE {
    fn create(&self) -> Box<dyn IButton> {
        Box::new(KdeButton {})
    }
}
impl Factory<Window> for KDE {
    fn create(&self) -> Box<Window> {
        Box::new(Window {})
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
