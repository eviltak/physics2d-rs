pub mod pool;

macro_rules! match_owned_fn_to_variants {
    ($enum_name:ident::{$($variant:ident),*}; $val:expr, $fn_name:ident$args:tt) => {
        match $val {
            $(
            $enum_name::$variant(_data) =>  _data.$fn_name$args,
            )*
        }
    };
}

macro_rules! match_borrow_fn_to_variants {
    ($enum_name:ident::{$($variant:ident),*}; $val:expr, $fn_name:ident$args:tt) => {
        match $val {
            $(
            $enum_name::$variant(ref _data) =>  _data.$fn_name$args,
            )*
        }
    };
}

macro_rules! match_mut_fn_to_variants {
    ($enum_name:ident::{$($variant:ident),*}; $val:expr, $fn_name:ident$args:tt) => {
        match $val {
            $(
            $enum_name::$variant(ref mut _data) =>  _data.$fn_name$args,
            )*
        }
    };
}

macro_rules! generate_match_owned_fn_macro_for_enum {
    ($enum_name:ident::{$($variant:ident),*}; $macro_name:ident) => {
        macro_rules! $macro_name {
            ($val:expr, $fn_name:ident$args:tt) => {
                match_owned_fn_to_variants!($enum_name::{$($variant),*}; $val, $fn_name$args)
            };
        }
    };
}

macro_rules! generate_match_borrow_fn_macro_for_enum {
    ($enum_name:ident::{$($variant:ident),*}; $macro_name:ident) => {
        macro_rules! $macro_name {
            ($val:expr, $fn_name:ident$args:tt) => {
                match_borrow_fn_to_variants!($enum_name::{$($variant),*}; $val, $fn_name$args)
            };
        }
    };
}

macro_rules! generate_match_mut_fn_macro_for_enum {
    ($enum_name:ident::{$($variant:ident),*}; $macro_name:ident) => {
        macro_rules! $macro_name {
            ($val:expr, $fn_name:ident$args:tt) => {
                match_mut_fn_to_variants!($enum_name::{$($variant),*}; $val, $fn_name$args)
            };
        }
    };
}

