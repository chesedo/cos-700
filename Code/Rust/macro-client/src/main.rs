mod abstract_factory;
mod abstract_factory_hand;
mod abstract_factory_r;
mod abstract_factory_r_hand;
mod abstract_factory_trait;
mod abstract_factory_trait_hand;

mod visitor_r;
mod visitor_r_hand;

mod gui;

fn main() {}

/// Makes a trait `t` be a subtriat for each factory `f` having one generic type for each type in `types`
/// Cannot be broken down further since macro rules cannot appear in the where clause
#[macro_export]
macro_rules! abstract_factory_r {
    ($v:vis $t:ident, $f:ident, $($types:ty),+) => {
        $v trait $t
        where
        $(Self: $f<$types>),* {}
    };
}

/// Call the local trait_expand! macro rule for each trait to concrete mapping seperated by a comma - allows a trailing comma
/// So each mapping is `trait => concrete_impelmentation`
#[macro_export]
macro_rules! traits_expansion {
    ($($traits:ty => $concretes:ident),+) => {
        traits_expansion!($($traits => $concretes,)*);
    };
    ($($traits:ty => $concretes:ident,)+) => {
        $(trait_expand!($traits, $concretes);)*
    }
}

#[cfg(test)]
mod tests {
    use macro_test_helpers::{expand, expand_cli};
    use pretty_assertions::assert_eq;

    #[test]
    fn abstract_factory() {
        expand!("abstract_factory");
    }

    #[test]
    fn abstract_factory_trait() {
        expand!("abstract_factory_trait");
    }

    #[test]
    fn abstract_factory_r() {
        expand!("abstract_factory_r");
    }

    #[test]
    fn visitor_r() {
        expand!("visitor_r");
    }
}
