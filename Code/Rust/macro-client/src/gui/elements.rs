pub trait Element {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
}

pub trait Button: Element {
    fn click(&self);
    fn get_text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

pub trait Input: Element {
    fn get_input(&self) -> String;
    fn set_input(&mut self, input: String);
}

pub enum Child {
    Button(Box<dyn Button>),
    Input(Box<dyn Input>),
}

pub struct Window {
    name: String,
    children: Vec<Child>,
}

impl Window {
    pub fn add_child(&mut self, child: Child) -> &mut Self {
        self.children.push(child);

        self
    }
    pub fn get_children(&self) -> &[Child] {
        &self.children
    }
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
}

impl From<Box<dyn Button>> for Child {
    fn from(button: Box<dyn Button>) -> Self {
        Child::Button(button)
    }
}

impl From<Box<dyn Input>> for Child {
    fn from(input: Box<dyn Input>) -> Self {
        Child::Input(input)
    }
}
