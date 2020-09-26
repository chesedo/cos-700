#[macro_export]
macro_rules! visitor {
    ($($types:tt),+) => {
        $crate::visitor!($($types,)+);
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
    // Factor out template cases
    ((&dyn $type:ident[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_trait_fn!((&dyn $type));
    };
    ((&$type:ident[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_trait_fn!((&$type));
    };

    // Handle minimal cases
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

    // Handle no helpers / default cases
    ((&dyn $type:ident[ helper_fn = false ])) => {
        $crate::visitor_trait_fn!((&dyn $type, no_default));
    };
    ((&$type:ident[ helper_fn = false ])) => {
        $crate::visitor_trait_fn!((&$type, no_default));
    };
    ((& dyn$type:ident, no_default)) => {
        $crate::visitor_trait_fn!($type, &dyn $type, no_default);
    };
    ((&$type:ident, no_default)) => {
        $crate::visitor_trait_fn!($type, &$type, no_default);
    };
    ($name:ident, $type:ty, no_default) => {
        paste::paste! {
            fn [<visit_ $name:snake>](&mut self, [<$name:snake>]: $type);
        }
    };
}

#[macro_export]
macro_rules! visitor_fn_helper {
    // Cases without a custom template
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

    // Cases with a custom template
    ((&dyn $type:tt[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_fn_helper!($type, &dyn $type, $tmpl);
    };
    ((&$type:ident[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_fn_helper!($type, &$type, $tmpl);
    };
    ($name:ident, $type:ty, $tmpl:ident) => {
        paste::paste! {
            pub fn [<visit_ $name:snake>]<V>(visitor: &mut V, [< $name:snake>]: $type)
            where
                V: Visitor + ?Sized,
            {
                $tmpl!([<$name:snake>], visitor);
            }
        }
    };

    // Handle no helpers cases - ie do nothing
    ((&dyn $type:ident[ helper_fn = false ])) => {};
    ((&$type:ident[ helper_fn = false ])) => {};
}

#[macro_export]
macro_rules! visitor_visitable {
    // Factor out template options
    ((&dyn $type:ident[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_visitable!((&dyn $type));
    };
    ((&$type:ident[ helper_tmpl = $tmpl:ident ])) => {
        $crate::visitor_visitable!((&$type));
    };

    // Factor out no helper / defauld options
    ((&dyn $type:ident[ helper_fn = false ])) => {
        $crate::visitor_visitable!((&dyn $type));
    };
    ((&$type:ident[ helper_fn = false ])) => {
        $crate::visitor_visitable!((&$type));
    };

    // Handle minimal cases
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
