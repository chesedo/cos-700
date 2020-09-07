use super::elements::{Element, IButton};

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
