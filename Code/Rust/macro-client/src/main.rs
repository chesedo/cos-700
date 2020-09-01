mod abstract_factory;
mod abstract_factory_hand;
mod abstract_factory_r;
mod abstract_factory_r_hand;

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

#[cfg(test)]
mod tests {
    use macro_test_helpers::expand;

    #[test]
    fn abstract_factory() {
        expand("abstract_factory");
    }

    #[test]
    fn abstract_factory_r() {
        expand("abstract_factory_r");
    }
}
