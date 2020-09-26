use macro_patterns_dec::abstract_factory;
pub trait Abstract: Factory<dyn Button> + Factory<Window> {}
