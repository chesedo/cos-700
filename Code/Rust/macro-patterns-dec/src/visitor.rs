#[macro_export]
macro_rules! visitor {
    ($($types:tt),+) => {
        visitor!($($types,)+)
    };
    ($($types:tt,)+) => {
        pub trait Visitor {
            $($crate::visitor_trait_fn!($types);)*
        }

        $($crate::visitor_fn_helper!($types);)*
    };
}

#[macro_export]
macro_rules! visitor_trait_fn {
    ((& dyn$type:ident)) => {
        $crate::visitor_trait_fn!($type, &dyn $type);
    };
    ((&$type:ident)) => {
        $crate::visitor_trait_fn!($type, &$type);
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: $type) {
                [<visit_ $name:lower>](self, [<$name:lower>])
            }
        }
    };
}

#[macro_export]
macro_rules! visitor_fn_helper {
    ((&dyn $type:ident)) => {
        $crate::visitor_fn_helper!($type, &dyn $type);
    };
    ((&$type:ident)) => {
        $crate::visitor_fn_helper!($type, &$type);
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            pub fn [<visit_ $name:lower>]<V>(_visitor: &mut V, [<_ $name:lower>]: $type)
            where
                V: Visitor + ?Sized,
            { }
        }
    };
}
