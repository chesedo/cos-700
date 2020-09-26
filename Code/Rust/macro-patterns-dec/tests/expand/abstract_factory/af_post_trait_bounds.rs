use macro_patterns_dec::abstract_factory;

abstract_factory!(
    pub Abstract: Factory<T> + Display + Debug,
    (
        Window,
        dyn Input
    )
);
