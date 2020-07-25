fn main() {
    {
        let s = String::from("string");
        let t = s;

        println!("String len is {}", s.len()); // borrow of moved value: `s`
    } // Compiler will 'drop' t here
}
