#[macro_export]
macro_rules! visitor {
    ($($types:tt),+) => {
        visitor!($($types,)+)
    };
    ($($types:tt,)+) => {
        pub trait Visitor {
            $(visitor_trait_fn!($types);)*
        }

        $(visitor_fn_helper!($types);)*
    };
}

#[macro_export]
macro_rules! visitor_trait_fn {
    (($type:ty)) => {
        fn visit_input(&mut self, input: $type) {
            visit_input(self, input)
        }
    };
}

#[macro_export]
macro_rules! visitor_fn_helper {
    (($type:ty)) => {
        pub fn visit_input<V>(_visitor: &mut V, _input: $type)
        where
            V: Visitor + ?Sized,
        {
        }
    };
}
