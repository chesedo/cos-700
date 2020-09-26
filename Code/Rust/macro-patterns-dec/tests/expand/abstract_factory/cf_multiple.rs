use macro_patterns_dec::concrete_factory;

macro_rules! factory_create_named {
    ($concrete:ty: $trait:ty) => {
        fn create(&self, name: String) -> Box<$trait> {
            Box::new(<$concrete>::new(name));
        }
    };
}
concrete_factory!(
    factory_create_named(
        impl Factory<T> for KDE {
            Window: Window,
            Group: Group,
            KDE::Button: dyn Button,
            Input: dyn KDE::Input,
        }
    )
);
