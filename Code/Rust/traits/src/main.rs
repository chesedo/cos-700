use std::fmt::Display;

trait Show {
    fn show(&self) -> String;

    // Method with a default implemetation
    fn show_size(&self) -> usize {
        self.show().chars().count()
    }
}

fn work<T: Show>(object: T) {
    println!("{}", object.show());
}

fn complex<T: Show + Display + PartialEq<U>, U: Display>(first: T, second: U) {
    if first == second {
        println!("{}({}) is equals {}", first, first.show(), second);
    }
}

fn complex_where<T, U>(first: T, second: U)
where
    T: Show + Display + PartialEq<U>,
    U: Display,
{
    // Same as complex
}

struct Tester {}

impl Show for Tester {
    fn show(&self) -> String {
        String::from("Tester")
    }
}

fn main() {
    let s = Tester {};

    work(s);
}
