trait Factory<T> {
    fn create(&self) -> T;
}

trait AbstactFactory: Factory<u32> + Factory<i64> {}

struct Numbers {}

impl Factory<u32> for Numbers {
    fn create(&self) -> u32 {
        5
    }
}

impl Factory<i64> for Numbers {
    fn create(&self) -> i64 {
        -2
    }
}

impl AbstactFactory for Numbers {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_trait() {
        unimplemented!();
    }

    #[test]
    fn u32_factory() {
        let n = Numbers {};
        let i: u32 = n.create();

        assert_eq!(i, 5);
    }

    #[test]
    fn i64_factory() {
        let n = Numbers {};
        let j: i64 = n.create();

        assert_eq!(j, -2);
    }
}
