fn main() {
    let mut s = String::from("string");
    take_borrow(&mut s);
    println!("String len is {}", s.len());
} // Compiler will add memory clean up code for s here
fn take_borrow(a: &mut String) {
    a.push_str("suffix");
}
