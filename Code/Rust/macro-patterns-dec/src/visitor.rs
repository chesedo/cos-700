#[macro_export]
macro_rules! visitor {
    ($($types:tt),+) => {
        visitor!($($types,)+)
    };
    ($($types:tt,)+) => {
        pub trait Visitor {
            $($crate::visitor_trait_fn!($types);)+
        }

        $($crate::visitor_fn_helper!($types);)+

        trait Visitable {
            fn apply(&self, visitor: &mut dyn Visitor);
        }

        $($crate::visitor_visitable!($types);)+
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
            fn [<visit_ $name:snake>](&mut self, [<$name:snake>]: $type) {
                [<visit_ $name:snake>](self, [<$name:snake>])
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
            pub fn [<visit_ $name:snake>]<V>(_visitor: &mut V, [<_ $name:snake>]: $type)
            where
                V: Visitor + ?Sized,
            { }
        }
    };
}

#[macro_export]
macro_rules! visitor_visitable {
    ((&dyn $type:ident)) => {
        $crate::visitor_visitable!($type, dyn $type);
    };
    ((&$type:ident)) => {
        $crate::visitor_visitable!($type, $type);
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            impl Visitable for $type {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.[<visit_ $name:snake>](self);
                }
            }
        }
    };
}
