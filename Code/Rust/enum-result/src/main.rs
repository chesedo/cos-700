enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn may_error() -> Result<i32, String> {
    Result::Err(String::from("Network down!"))
}

fn error_explicit_handle() {
    let r = may_error();
    let r = match r {
        Result::Ok(result) => result,
        Result::Err(error) => panic!("Operation failed: {}", error),
    };
}
fn error_short_handle() {
    let r = may_error().expect("Operation failed");
}

fn error_explicit_propogation() -> Result<i32, String> {
    let r = match may_error() {
        Result::Ok(result) => result,
        Result::Err(error) => return Err(error),
    };

    Ok(2)
}
fn error_short_propogation() -> Result<i32, String> {
    let r = may_error()?;

    Ok(2)
}

fn main() {
    error_short_handle();
}
