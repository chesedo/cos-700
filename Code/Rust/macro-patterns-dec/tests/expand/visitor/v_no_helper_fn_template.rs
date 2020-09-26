use macro_patterns_dec::visitor;

visitor!(
    (&Window[ helper_fn = false ]),
    (&dyn Group[ helper_fn = false ]),
);
