#[macro_export]
macro_rules! visitor {
    // For the macro meta, `:meta` is not used because of
    // https://github.com/rust-lang/rust/issues/49629
    (
        $(
            $(#[$($attr:tt)+])*
            $($types:ident)+,
        )+
    ) => {
        pub trait Visitor {
            $(
                $crate::visitor_trait_fn!(
                    $(#[$($attr)+])*
                    $($types)+
                );
            )+
        }

        $(
            $crate::visitor_fn_helper!(
                $(#[$($attr)+])*
                $($types)+
            );
        )+

        trait Visitable {
            fn apply(&self, visitor: &mut dyn Visitor);
        }

        $(
            $crate::visitor_visitable!($($types)+);
        )+
    };
}

#[macro_export]
macro_rules! visitor_trait_fn {
    // Unstack attributes and redirect
    (
        #[helper_fn = false]
        $($types:ident)+
    ) => {
        $crate::visitor_trait_fn!(__no_default, $($types)+);
    };

    (
        #[$($_attr_head:tt)+]
        $(#[$($attr_tail:tt)+])*
        $($types:ident)+
    ) => {
        $crate::visitor_trait_fn!(
            $(#[$($attr_tail)+])*
            $($types)+
        );
    };

    // Handle no helpers / default case
    (__no_default, dyn $type:ty) => {
        paste::paste! {
            $crate::visitor_trait_fn!(__no_default, [<$type:snake>], dyn $type);
        }
    };
    (__no_default, $type:ty) => {
        paste::paste! {
            $crate::visitor_trait_fn!(__no_default, [<$type:snake>], $type);
        }
    };
    (__no_default, $name:ident, $type:ty) => {
        paste::paste! {
            fn [<visit_ $name>](&mut self, $name: &$type);
        }
    };

    // Handle default case
    (dyn $type:ty) => {
        paste::paste! {
            $crate::visitor_trait_fn!([<$type:snake>], dyn $type);
        }
    };
    ($type:ty) => {
        paste::paste! {
            $crate::visitor_trait_fn!([<$type:snake>], $type);
        }
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            fn [<visit_ $name>](&mut self, $name: &$type) {
                [<visit_ $name>](self, $name)
            }
        }
    };
}

#[macro_export]
macro_rules! visitor_fn_helper {
    // Unstack attributes and redirect
    (
        #[helper_fn = false]
        $($types:ident)+
    ) => {};
    (
        #[helper_tmpl = $tmpl:ident]
        $($types:ident)+
    ) => {
        $crate::visitor_fn_helper!(__tmpl, $tmpl, $($types)+);
    };

    (
        #[$($_attr_head:tt)+]
        $(#[$($attr_tail:tt)+])*
        $($types:ident)+
    ) => {
        $crate::visitor_fn_helpe!(
            $(#[$($attr_tail)+])*
            $($types)+
        );
    };

    // Handle case with custom template
    (__tmpl, $tmpl:ident, dyn $type:ty) => {
        paste::paste! {
            $crate::visitor_fn_helper!(__tmpl, $tmpl, [<$type:snake>], dyn $type);
        }
    };
    (__tmpl, $tmpl:ident, $type:ty) => {
        paste::paste! {
            $crate::visitor_fn_helper!(__tmpl, $tmpl, [<$type:snake>], $type);
        }
    };
    (__tmpl, $tmpl:ident, $name:ident, $type:ty) => {
        paste::paste! {
            pub fn [<visit_ $name>]<V>(visitor: &mut V, $name: &$type)
            where
                V: Visitor + ?Sized,
            {
                $tmpl!($name, visitor);
            }
        }
    };

    // Handle default case
    (dyn $type:ty) => {
        paste::paste! {
            $crate::visitor_fn_helper!([<_ $type:snake>], dyn $type);
        }
    };
    ($type:ty) => {
        paste::paste! {
            $crate::visitor_fn_helper!([<_ $type:snake>], $type);
        }
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            pub fn [<visit $name>]<V>(_visitor: &mut V, $name: &$type)
            where
                V: Visitor + ?Sized,
            { }
        }
    };
}

#[macro_export]
macro_rules! visitor_visitable {
    // Has no special cases
    (dyn $type:ty) => {
        paste::paste! {
            $crate::visitor_visitable!([<$type:snake>], dyn $type);
        }
    };
    ($type:ty) => {
        paste::paste! {
            $crate::visitor_visitable!([<$type:snake>], $type);
        }
    };
    ($name:ident, $type:ty) => {
        paste::paste! {
            impl Visitable for $type {
                fn apply(&self, visitor: &mut dyn Visitor) {
                    visitor.[<visit_ $name>](self);
                }
            }
        }
    };
}
