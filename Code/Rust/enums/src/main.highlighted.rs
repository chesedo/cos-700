enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let age = Option::Some(5);

    match age {
        Option::Some(|\inputh{value}|) => println!("Age = {}", |\inputh{value}|),
        Option::None => println!("Age is unknown"),
    }

    let |\contexth{valid}| = if let Option::Some(|\outputh{\_}|) = age {
        println!("Age is set");
        true
    } else {
        false
    };
}
