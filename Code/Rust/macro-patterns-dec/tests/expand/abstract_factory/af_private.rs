use macro_patterns_dec::abstract_factory;

abstract_factory!(
    trait InternalAbstract: InternalFactory<T> + Display {
        Window,
        Group,
    }
);
