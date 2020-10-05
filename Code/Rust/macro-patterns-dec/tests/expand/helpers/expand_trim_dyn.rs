use macro_patterns_dec::expand_trim_dyn;

macro_rules! template {
    ($name:ident, $type:ty) => {
        struct TestStruct {
            $name: $type,
        }
    };
}

expand_trim_dyn!(template, Display);

expand_trim_dyn!(template, dyn Debug);
