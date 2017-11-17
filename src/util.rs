
macro_rules! match_fn_to_variants {
    ($enum_name:ident::{$($variant:ident),*}; $val:expr, $fn_name:ident$args:tt) => {
        match $val {
            $(
            $enum_name::$variant(ref _data) =>  _data.$fn_name$args,
            )*
        }
    };
}
