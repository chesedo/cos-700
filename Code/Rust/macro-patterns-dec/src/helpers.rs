#[macro_export]
macro_rules! expand {
    ($tpml:ident($($items:tt),+)) => {
        $($tpml!($items);)+
    };
    ($tpml:ident($($items:tt,)+)) => {
        $($tpml!($items);)+
    };
}

#[macro_export]
macro_rules! expand_trim_dyn {
    ($tmpl:path, dyn $type:ty $(, $extra:path)*) => {
        paste::paste! {
            $tmpl!([<$type:snake>], dyn $type $(, $extra)*);
        }
    };
    ($tmpl:path, $type:ty $(, $extra:path)*) => {
        paste::paste! {
            $tmpl!([<$type:snake>], $type $(, $extra)*);
        }
    };
}

#[cfg(test)]
mod test {
    //! To test compilation

    use std::fmt::Display;

    #[macro_export]
    macro_rules! template {
        ($name:ident, $type:ty) => {
            paste::paste! {
                struct [<Test $name>] {
                    $name: $type,
                }
            }
        };
    }
    macro_rules! make_struct {
        ($type:ty) => {
            expand_trim_dyn!($crate::template, $type);
        };
    }

    make_struct!(str);
    expand_trim_dyn!(template, dyn Display);
}
