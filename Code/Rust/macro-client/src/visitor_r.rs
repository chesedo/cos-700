use crate::gui::elements::{Button, Child, Input, Window};
#[allow(unused_imports)]
use macro_patterns_dec::visitor;
use std::fmt;

macro_rules! children_walker {
    ($var_name:ident, $visitor_name:ident) => {
        $var_name.get_children().iter().for_each(|child| {
            match child {
                Child::Button(button) => $visitor_name.visit_button(button.as_ref()),
                Child::Input(input) => $visitor_name.visit_input(input.as_ref()),
            };
        });
    };
}

visitor!(
    dyn Button,
    dyn Input,

    #[helper_tmpl = children_walker]
    Window,
);

struct NameVisitor {
    names: Vec<String>,
}

impl NameVisitor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NameVisitor { names: Vec::new() }
    }
}

impl fmt::Display for NameVisitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.names.join(", "))
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
