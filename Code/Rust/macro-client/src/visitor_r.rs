#[allow(unused_imports)]
use crate::gui::elements::{Button, Child, Element, Input, Window};
use macro_patterns_dec::visitor;
use std::fmt;
use std::ops::Deref;

macro_rules! children_walker {
    ($var_name:ident, $visitor_name:ident) => {
        $var_name.get_children().iter().for_each(|child| {
            match child {
                Child::Button(button) => $visitor_name
                    .visit_button(button.read().expect("Button is no longer readable").deref()),
                Child::Input(input) => $visitor_name
                    .visit_input(input.read().expect("Input is no longer readable").deref()),
            };
        });
    };
}

visitor!(
    dyn Element,
    dyn Button,
    dyn Input,

    #[helper_tmpl = children_walker]
    Window,
);

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
    fn visit_button(&mut self, button: &dyn Button) {
        self.names.push(button.get_name().to_string());
    }

    fn visit_input(&mut self, input: &dyn Input) {
        self.names
            .push(format!("{} ({})", input.get_name(), input.get_input()));
    }

    fn visit_window(&mut self, window: &Window) {
        self.names.push(window.get_name().to_string());

        visit_window(self, window);
    }
}
