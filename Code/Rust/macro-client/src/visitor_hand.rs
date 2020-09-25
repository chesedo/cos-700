use crate::gui::elements::{Child, Element, IButton, IInput, Window};
use std::fmt;
use std::ops::Deref;

pub trait Visitor {
    fn visit_element(&mut self, element: &dyn Element) {
        visit_element(self, element)
    }
    fn visit_button(&mut self, button: &dyn IButton) {
        visit_button(self, button)
    }
    fn visit_input(&mut self, input: &dyn IInput) {
        visit_input(self, input)
    }
    fn visit_window(&mut self, window: &Window) {
        visit_window(self, window)
    }
}

pub fn visit_element<V>(_visitor: &mut V, _element: &dyn Element)
where
    V: Visitor + ?Sized,
{
}

pub fn visit_button<V>(_visitor: &mut V, _button: &dyn IButton)
where
    V: Visitor + ?Sized,
{
}

pub fn visit_input<V>(_visitor: &mut V, _input: &dyn IInput)
where
    V: Visitor + ?Sized,
{
}

pub fn visit_window<V>(visitor: &mut V, window: &Window)
where
    V: Visitor + ?Sized,
{
    window.get_children().iter().for_each(|child| {
        match child {
            Child::Button(button) => {
                visitor.visit_button(button.read().expect("Button is no longer readable").deref())
            }
            Child::Input(input) => {
                visitor.visit_input(input.read().expect("Input is no longer readable").deref())
            }
        };
    });
}

trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}

impl Visitable for Window {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_window(self);
    }
}

impl Visitable for dyn Element {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_element(self);
    }
}

impl Visitable for dyn IButton {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_button(self);
    }
}

impl Visitable for dyn IInput {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_input(self);
    }
}

struct VisitorName {
    names: Vec<String>,
}

impl VisitorName {
    pub fn new() -> Self {
        VisitorName { names: Vec::new() }
    }
}

impl fmt::Display for VisitorName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.names.join(", "))
    }
}

impl Visitor for VisitorName {
    fn visit_button(&mut self, button: &dyn IButton) {
        self.names.push(button.get_name().to_string());
    }

    fn visit_input(&mut self, input: &dyn IInput) {
        self.names
            .push(format!("{} ({})", input.get_name(), input.get_input()));
    }

    fn visit_window(&mut self, window: &Window) {
        self.names.push(window.get_name().to_string());

        visit_window(self, window);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::elements::{AWrap, Child};
    use crate::gui::kde::{Input, KdeButton};
    use std::sync::{Arc, RwLock};
    use std::thread;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn visit_button() {
        let button: &dyn IButton = &KdeButton::new(String::from("Some Button"));

        let mut visitor = VisitorName::new();

        button.apply(&mut visitor);

        assert_eq!(visitor.to_string(), "Some Button");
    }

    #[test]
    fn visit_window() -> Result {
        let window = Arc::new(RwLock::new(Window::new(String::from("Holding window"))));
        let button: AWrap<dyn IButton> =
            Arc::new(RwLock::new(KdeButton::new(String::from("Some Button"))));
        let input: AWrap<dyn IInput> =
            Arc::new(RwLock::new(Input::new(String::from("Some Input"))));

        window
            .write()
            .expect("Failed to get writable window")
            .add_child(Child::from(button))
            .add_child(Child::from(input.clone()));

        input
            .write()
            .expect("Failed to get writable input")
            .set_input(String::from("John Doe"));

        let clone = window.clone();

        let handle = thread::spawn(move || {
            let mut visitor = VisitorName::new();

            clone
                .read()
                .expect("Clone is no longer readable")
                .apply(&mut visitor);

            assert_eq!(
                visitor.to_string(),
                "Holding window, Some Button, Some Input (Mary Doe)"
            );
        });

        let mut visitor = VisitorName::new();

        window
            .read()
            .expect("Clone is no longer readable")
            .apply(&mut visitor);

        assert_eq!(
            visitor.to_string(),
            "Holding window, Some Button, Some Input (John Doe)"
        );

        input
            .write()
            .expect("Failed to get writable input")
            .set_input(String::from("Mary Doe"));

        handle.join().expect("Test failed");

        Ok(())
    }
}
