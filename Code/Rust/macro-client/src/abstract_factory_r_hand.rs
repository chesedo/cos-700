use crate::abstract_factory_r;

trait Factory<T: Element> {
    fn create(&self) -> T;
}
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

trait Gui
where
    Self: Factory<Button>,
    Self: Factory<Window>,
{
}

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
