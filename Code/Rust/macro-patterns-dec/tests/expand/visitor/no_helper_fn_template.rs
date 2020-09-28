use macro_patterns_dec::visitor;

visitor!(
    #[helper_fn = false]
    Window,

    #[helper_fn = false]
    dyn Group,
);
