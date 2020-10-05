#[macro_export]
macro_rules! visitor {
    // For the macro meta, `:meta` is not used because of
    // https://github.com/rust-lang/rust/issues/49629
    (
        $(
            $(#$attr:tt)*
            $($types:ident)+,
        )+
    ) => {
        pub trait Visitor {
            $(
                $crate::visitor_trait_fn!(
                    $(#$attr)*
                    $($types)+
                );
            )+
        }

        $(
            $crate::visitor_fn_helper!(
                $(#$attr)*
                $($types)+
            );
        )+

        trait Visitable {
            fn apply(&self, visitor: &mut dyn Visitor);
        }

        $(
            $crate::expand_trim_dyn!($crate::visitor_visitable_impl, $($types)+);
        )+
    };
}

#[macro_export]
macro_rules! visitor_trait_fn {
    // Unstack attributes and redirect
    (
        #[helper_fn = false]
        $($type:ident)+
    ) => {
        $crate::expand_trim_dyn!($crate::visitor_trait_fn_no_default, $($type)+);
    };

    (
        #$_attr_head:tt
        $(#$attr_tail:tt)*
        $($type:ident)+
    ) => {
        $crate::visitor_trait_fn!(
            $(#$attr_tail)*
            $($type)+
        );
    };

    // Handle default case
    ($($type:ident)+) => {
        $crate::expand_trim_dyn!($crate::visitor_trait_fn_default, $($type)+);
    };
}

#[macro_export]
macro_rules! visitor_trait_fn_no_default {
    ($name:ident, $type:ty) => {
        paste::paste! {
            fn [<visit_ $name>](&mut self, $name: &$type);
        }
    };
}

#[macro_export]
macro_rules! visitor_trait_fn_default {
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
        $($type:ident)+
    ) => {};
    (
        #[helper_tmpl = $tmpl:ident]
        $($type:ident)+
    ) => {
        $crate::expand_trim_dyn!($crate::visitor_fn_helper_custom, $($type)+, $tmpl);
    };

    (
        #$_attr_head:tt
        $(#$attr_tail:tt)*
        $($type:ident)+
    ) => {
        $crate::visitor_fn_helper!(
            $(#$attr_tail)*
            $($type)+
        );
    };

    // Handle default case
    ($($type:ident)+) => {
        $crate::expand_trim_dyn!($crate::visitor_fn_helper_empty, $($type)+);
    };
}

#[macro_export]
macro_rules! visitor_fn_helper_custom {
    ($name:ident, $type:ty, $tmpl:path) => {
        paste::paste! {
            pub fn [<visit_ $name>]<V>(visitor: &mut V, $name: &$type)
            where
                V: Visitor + ?Sized,
            {
                $tmpl!($name, visitor);
            }
        }
    };
}

#[macro_export]
macro_rules! visitor_fn_helper_empty {
    ($name:ident, $type:ty) => {
        paste::paste! {
            pub fn [<visit_ $name>]<V>(_visitor: &mut V, [<_ $name>]: &$type)
            where
                V: Visitor + ?Sized,
            { }
        }
    };
}

#[macro_export]
macro_rules! visitor_visitable_impl {
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

#[cfg(test)]
mod test {
    //! Just to test if compilation is successful

    use std::fmt::{Debug, Display};
    use std::string::ToString;

    macro_rules! custom_template {
        ($var_name:ident, $visitor_name:ident) => {
            $visitor_name.visit_debug($var_name);
        };
    }

    visitor!(
        u32,

        #[helper_fn = false]
        i32,

        dyn Display,

        #[helper_tmpl = custom_template]
        dyn Debug,

        #[helper_tmpl = custom_template]
        #[helper_fn = false]
        dyn ToString,
    );
}
