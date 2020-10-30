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
    // Same as complex with easier to read trait bounds
}

struct Tester {}

impl Show for Tester {
    fn show(&self) -> String {
        String::from("Tester")
    }
}

fn work_tester(object: Tester) {
    println!("{}", object.show());
}
fn work_string(object: String) {
    println!("{}", object.show());
}

fn main() {
    let s = Tester {};

    work(s);
}
