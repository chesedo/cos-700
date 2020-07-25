fn main() {
    let s = String::from("string");
    take_borrow(&s);

    println!("String len is {}", s.len());
} // Compiler will 'drop' s here

fn take_borrow(a: &String) {
    a.push_str("suffix"); // cannot borrow *a as mutable
} // a is borrowed and wiil therefore not be dropped
