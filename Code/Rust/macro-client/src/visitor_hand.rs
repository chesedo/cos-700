#[allow(unused_imports)]
use macro_patterns::visitor;
use std::fmt;

use crate::gui::elements::{Button, Child, Input, Window};

// Abstract visitor for `Button`, `Input` and `Window`
pub trait Visitor {
    fn visit_button(&mut self, button: &dyn Button) {
        visit_button(self, button)
    }
    fn visit_input(&mut self, input: &dyn Input) {
        visit_input(self, input)
    }
    fn visit_window(&mut self, window: &Window) {
        visit_window(self, window)
    }
}

// Helper functions for transversing a hierarchical data structure
pub fn visit_button<V>(_visitor: &mut V, _button: &dyn Button)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_input<V>(_visitor: &mut V, _input: &dyn Input)
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
            Child::Button(button) => visitor.visit_button(button.as_ref()),
            Child::Input(input) => visitor.visit_input(input.as_ref()),
        };
    });
}

// Extends each element with the reflective `apply` method
trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}

impl Visitable for dyn Button {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_button(self);
    }
}
impl Visitable for dyn Input {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_input(self);
    }
}
impl Visitable for Window {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_window(self);
    }
}

struct VisitorName {
    names: Vec<String>,
}

impl VisitorName {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VisitorName { names: Vec::new() }
    }
}
impl Visitor for VisitorName {
    fn visit_button(&mut self, button: &dyn Button) {
        self.names.push(button.get_name().to_string());
    }
    fn visit_input(&mut self, input: &dyn Input) {
        self.names
            .push(format!("{} ({})", input.get_name(), input.get_input()));
    }
}

impl fmt::Display for VisitorName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.names.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::brand_elements;
    use crate::gui::elements::{Child, Element};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn visit_button() {
        let button: &dyn Button = &brand_elements::BrandButton::new(String::from("Some Button"));

        let mut visitor = VisitorName::new();

        button.apply(&mut visitor);

        assert_eq!(visitor.to_string(), "Some Button");
    }

    #[test]
    fn visit_window() -> Result {
        let mut window = Box::new(Window::new(String::from("Holding window")));
        let button: Box<dyn Button> = Box::new(brand_elements::BrandButton::new(String::from(
            "Some Button",
        )));
        let mut input: Box<dyn Input> =
            Box::new(brand_elements::BrandInput::new(String::from("Some Input")));

        input.set_input(String::from("John Doe"));

        window
            .add_child(Child::from(button))
            .add_child(Child::from(input));

        let mut visitor = VisitorName::new();

        window.apply(&mut visitor);

        assert_eq!(visitor.to_string(), "Some Button, Some Input (John Doe)");

        Ok(())
    }
}
