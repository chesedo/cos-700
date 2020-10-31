mod abstract_factory_r;
mod abstract_factory_r_hand;
mod abstract_factory_trait;
mod abstract_factory_trait_hand;

mod visitor_r;
mod visitor_r_hand;

mod gui;

fn main() {}

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
