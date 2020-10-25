use library::{reflect, reflect_two, GetType};

trait GetType {
    fn get_type(&self) -> String;
}

|\functionh{reflect}|!(|\inputh{2 + 3, 5}|);

#[derive(|\functionh{GetType}|)]
|\contexth{struct SomeStruct}|;
|\outputh{impl GetType for SomeStruct \{ ... \}}|

#[|\functionh{reflect\_two}|(|\inputh{multiple => tokens}|)]
|\contexth{fn some\_function() \{\}}|

fn main() {
    let s = SomeStruct {};
    println!("{}", s.get_type());
}
