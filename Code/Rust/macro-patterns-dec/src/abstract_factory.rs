/// Makes a trait `t` be a subtriat for each factory `f` having one generic type for each type in `types`
/// Cannot be broken down further since macro rules cannot appear in the where clause
#[macro_export]
macro_rules! abstract_factory {
    ($v:vis trait $t:ident: $f:ident<T> $(+ $post:tt)*, ($($types:ty),+)) => {
        $crate::abstract_factory!($v trait $t: $f<T> $(+ $post)*, ($($types,)+));
    };
    ($v:vis trait $t:ident: $f:ident<T> $(+ $post:tt)*, ($($types:ty,)+)) => {
        $v trait $t: $($f<$types> +)+ $($post +)*
        {}
    };
}
