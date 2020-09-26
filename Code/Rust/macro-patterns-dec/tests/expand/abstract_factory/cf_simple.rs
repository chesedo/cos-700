use macro_patterns_dec::concrete_factory;

macro_rules! factory_create {
    ($concrete:ident: $trait:ty) => {
        fn create(&self) -> $trait {
            $concrete::default();
        }
    };
}
concrete_factory!(
    factory_create(
        impl Factory<T> for KDE {
            Window: Window,
        }
    )
);
