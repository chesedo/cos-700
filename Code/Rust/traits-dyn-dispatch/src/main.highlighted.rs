trait Show {
    fn show(&self) -> String;
}

fn work(object: &dyn |\functionh{Show}|) {
    println!("{}", object.show());
}

fn main() {
    let s = Tester {};

    work(&s);
}

struct Tester {}

impl Show for Tester {
    fn show(&self) -> String {
        String::from("Tester")
    }
}
