enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let age = Option::Some(5);

    match age {
        Option::Some(value) => println!("Age = {}", value),
        Option::None => println!("Age is unknown"),
    }

    let valid = if let Option::Some(_) = age {
        println!("Age is set");
        true
    } else {
        false
    };
}
