fn main() {
    {
        let s = String::from("string");
        let t = s;

        println!("String len: {}", s.len()); |\errorh{borrow of moved value: `s`}|
    } // Compiler will 'drop' t here
}
