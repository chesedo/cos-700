use macro_patterns_dec::concrete_factory;
impl Factory<Window> for KDE {
    fn create(&self) -> Window {
        Window::default();
    }
}
