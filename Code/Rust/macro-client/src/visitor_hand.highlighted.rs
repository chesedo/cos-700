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

struct NameVisitor {
    names: Vec<String>,
}
impl NameVisitor {
    pub fn new() -> Self {
        NameVisitor { names: Vec::new() }
    }
}
impl Visitor for NameVisitor {
    fn visit_button(&mut self, button: &dyn Button) {
        self.names.push(button.get_name().to_string());
    }
    fn visit_input(&mut self, input: &dyn Input) {
        self.names
            .push(format!("{} ({})", input.get_name(), input.get_input()));
    }
}
impl fmt::Display for NameVisitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.names.join(", "))
    }
}
