fn main() {
    let s = String::from("string");
    take_ownership(s);

    println!("String len is {}", s.len()); // borrow of moved value: `s`
}

fn take_ownership(a: String) {
    // some code working on a
} // Compiler will 'drop' a here
