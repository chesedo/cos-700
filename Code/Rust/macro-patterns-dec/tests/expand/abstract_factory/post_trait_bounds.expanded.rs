use macro_patterns_dec::abstract_factory;
pub trait Abstract: Factory<Window> + Factory<dyn Input> + Display + Debug {}
