pub trait Element {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);
}

pub trait IButton: Element {
    fn click(&self);
    fn get_text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

pub trait IInput: Element {
    fn get_input(&self) -> String;
    fn set_input(&mut self, input: String);
}

pub struct Window {
    name: String,
}

impl Element for Window {
    fn new(name: String) -> Self {
        Window { name }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
