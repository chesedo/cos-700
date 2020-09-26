use macro_patterns_dec::concrete_factory;

// See https://github.com/rust-lang/rust/issues/52307 why this needs <$concrete>
macro_rules! factory_create {
    ($concrete:ty: $trait:ty) => {
        fn create(&self) -> $trait {
            <$concrete>::default();
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
