use super::elements;

pub struct BrandButton {
    name: String,
    text: String,
}

impl elements::Element for BrandButton {
    fn new(name: String) -> Self {
        BrandButton {
            name,
            text: String::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl elements::Button for BrandButton {
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

pub struct BrandInput {
    name: String,
    input: String,
}

impl elements::Element for BrandInput {
    fn new(name: String) -> Self {
        BrandInput {
            name,
            input: String::new(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl elements::Input for BrandInput {
    fn get_input(&self) -> String {
        self.input.to_owned()
    }
    fn set_input(&mut self, input: String) {
        self.input = input
    }
}
