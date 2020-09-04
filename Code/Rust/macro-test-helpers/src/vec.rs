macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[allow(dead_code)]
fn values() -> Vec<i32> {
    vec![1, 2, 3]
}
