use library::{reflect, reflect_two, GetType};

trait GetType {
    fn get_type(&self) -> String;
}

reflect!(2 + 3, 5);

#[derive(GetType)]
struct SomeStruct;
// impl GetType for SomeStruct { ... }

#[reflect_two(multiple => tokens)]
fn some_function() {}

fn main() {
    let s = SomeStruct {};
    println!("{}", s.get_type());
}
