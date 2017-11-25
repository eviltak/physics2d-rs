
macro_rules! match_fn_to_variants {
    ($enum_name:ident::{$($variant:ident),*}; $val:expr, $fn_name:ident$args:tt) => {
        match $val {
            $(
            $enum_name::$variant(ref _data) =>  _data.$fn_name$args,
            )*
        }
    };
}

macro_rules! generate_match_fn_macro_for_enum {
    ($enum_name:ident::{$($variant:ident),*}; $macro_name:ident) => {
        macro_rules! $macro_name {
            ($val:expr, $fn_name:ident$args:tt) => {
                match_fn_to_variants!($enum_name::{$($variant),*}; $val, $fn_name$args)
            };
        }
    };
}
