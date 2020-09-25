use macro_patterns_dec::expand;

macro_rules! template {
    ({$i:ident => $n:expr}) => {
        const $i: i32 = $n;
    };
}

expand!(
    template(
        {ONE => 1},
        {TWO => 2},
        {THREE => 3},
    )
)
