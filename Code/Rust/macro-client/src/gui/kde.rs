use super::elements::{Element, IButton, IInput};

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
    fn name(&self) -> &str {
        &self.name
    }
}

impl IInput for Input {
    fn get_input(&self) -> String {
        self.input.to_owned()
    }
}
