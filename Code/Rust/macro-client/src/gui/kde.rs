use super::elements::{Element, IButton, IInput};

pub struct KdeButton {
    name: String,
    text: String,
}

impl Element for KdeButton {
    fn new(name: String) -> Self {
        KdeButton {
            name,
            text: String::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl IButton for KdeButton {
    fn click(&self) {
        unimplemented!()
    }
    fn get_text(&self) -> &str {
        &self.text
    }
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Input {
    name: String,
    input: String,
}

impl Element for Input {
    fn new(name: String) -> Self {
        Input {
            name,
            input: String::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl IInput for Input {
    fn get_input(&self) -> String {
        self.input.to_owned()
    }
    fn set_input(&mut self, input: String) {
        self.input = input
    }
}
