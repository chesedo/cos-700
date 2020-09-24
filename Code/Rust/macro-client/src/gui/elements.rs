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

pub enum Child {
    Button(Box<dyn IButton>),
    Input(Box<dyn IInput>),
}

impl From<Box<dyn IButton>> for Child {
    fn from(button: Box<dyn IButton>) -> Self {
        Child::Button(button)
    }
}

impl From<Box<dyn IInput>> for Child {
    fn from(input: Box<dyn IInput>) -> Self {
        Child::Input(input)
    }
}

pub struct Window {
    name: String,
    children: Vec<Child>,
}

impl Element for Window {
    fn new(name: String) -> Self {
        Window {
            name,
            children: Vec::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Window {
    pub fn add_child(&mut self, child: Child) {
        self.children.push(child);
    }
    pub fn get_children(&self) -> &[Child] {
        &self.children
    }
}
