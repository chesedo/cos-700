mod abstract_factory;
mod abstract_factory_hand;

fn main() {}

#[cfg(test)]
mod tests {
    use macro_test_helpers::expand;

    #[test]
    fn abstract_factory() {
        expand("abstract_factory");
    }
}
