#[macro_export]
macro_rules! expand {
    ($tpml:ident($($items:tt),+)) => {
        $($tpml!($items);)+
    };
    ($tpml:ident($($items:tt,)+)) => {
        $($tpml!($items);)+
    };
}
