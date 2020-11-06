trait |\functionh{Show}| {
    fn show(&self) -> String;
    // Method with a default implemetation
    fn show_size(&self) -> usize {
        self.show().chars().count()
    }
}

fn work<T: |\functionh{Show}|>(object: T) {
    println!("{}", object.show());
}

struct Tester {}
impl |\functionh{Show}| for Tester {
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

impl Show for String {
    fn show(&self) -> String {
        self.clone()
    }
}

fn main() {
    let s = Tester {};

    work(s);
}
