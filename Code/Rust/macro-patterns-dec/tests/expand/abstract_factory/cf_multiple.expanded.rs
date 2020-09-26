use macro_patterns_dec::concrete_factory;
impl Factory<Window> for KDE {
    fn create(&self, name: String) -> Box<Window> {
        Box::new(<Window>::new(name));
    }
}
impl Factory<Group> for KDE {
    fn create(&self, name: String) -> Box<Group> {
        Box::new(<Group>::new(name));
    }
}
impl Factory<dyn Button> for KDE {
    fn create(&self, name: String) -> Box<dyn Button> {
        Box::new(<KDE::Button>::new(name));
    }
}
impl Factory<dyn KDE::Input> for KDE {
    fn create(&self, name: String) -> Box<dyn KDE::Input> {
        Box::new(<Input>::new(name));
    }
}
