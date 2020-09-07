pub trait Element {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
}

pub trait IButton: Element {
    fn click(&self);
}

pub struct Window {
    name: String,
}

impl Element for Window {
    fn new(name: String) -> Self {
        Window { name }
    }
    fn name(&self) -> &str {
        &self.name
    }
}
