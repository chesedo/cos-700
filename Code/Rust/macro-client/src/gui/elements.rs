use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub type Wrap<T> = Rc<RefCell<T>>;
pub type AWrap<T> = Arc<RwLock<T>>;

pub trait Element: Send + Sync {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);
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
    Button(AWrap<dyn Button>),
    Input(AWrap<dyn Input>),
}

impl From<AWrap<dyn Button>> for Child {
    fn from(button: AWrap<dyn Button>) -> Self {
        Child::Button(button)
    }
}

impl From<AWrap<dyn Input>> for Child {
    fn from(input: AWrap<dyn Input>) -> Self {
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
    pub fn add_child(&mut self, child: Child) -> &mut Self {
        self.children.push(child);

        self
    }
    pub fn get_children(&self) -> &[Child] {
        &self.children
    }
}
