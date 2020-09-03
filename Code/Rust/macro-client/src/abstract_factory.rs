use macro_patterns::abstract_factory;

trait Element {
    fn name(&self) -> &str;
}

struct Button {}

impl Element for Button {
    fn name(&self) -> &str {
        "Button"
    }
}

struct Window {}

impl Element for Window {
    fn name(&self) -> &str {
        "Window"
    }
}

trait Factory<T: Element> {
    fn create(&self) -> T;
}

abstract_factory!(Gui, Factory, Button, Window);

struct KDE {}

impl Gui for KDE {}

impl Factory<Button> for KDE {
    fn create(&self) -> Button {
        Button {}
    }
}
impl Factory<Window> for KDE {
    fn create(&self) -> Window {
        Window {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_factory() {
        let factory = KDE {};
        let actual: Button = factory.create();

        assert_eq!(actual.name(), "Button");
    }

    #[test]
    fn window_factory() {
        let factory = KDE {};
        let actual: Window = factory.create();

        assert_eq!(actual.name(), "Window");
    }
}
