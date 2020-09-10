mod abstract_factory;
mod abstract_factory_hand;
mod abstract_factory_r;
mod abstract_factory_r_hand;
mod abstract_factory_trait;
mod abstract_factory_trait_hand;

mod gui;

fn main() {}

/// Makes a trait `t` be a subtriat for each factory `f` having one generic type for each type in `types`
/// Cannot be broken down further since macro rules cannot appear in the where clause
#[macro_export]
macro_rules! abstract_factory_r {
    ($t:ident, $f:ident, $($types:ty),+) => {
        trait $t
        where
        $(Self: $f<$types>),* {}
    };
}

/// Call the local factory! macro rule for each trait to concrete mapping seperated by a comma
/// So each mapping is `trait => concrete_impelmentation`
#[macro_export]
macro_rules! concrete_factory_r {
    ($($traits:ty => $concretes:ident),+) => {
        $(factory!($traits, $concretes);)*
    };
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
}
