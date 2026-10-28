#![allow(non_upper_case_globals)]

#[doc(no_inline)]
pub use rodetc_span::sym::*;

macro_rules! val {
    ($name:ident) => {
        stringify!($name)
    };
    ($name:ident $value:literal) => {
        $value
    };
}

macro_rules! generate {
    ($($name:ident $(: $value:literal)? ,)*) => {
        /// To be supplied to `rodetc_interface::Config`
        pub const EXTRA_SYMBOLS: &[&str] = &[
            $(
                val!($name $($value)?),
            )*
        ];

        $(
            pub const $name: rodetc_span::Symbol = rodetc_span::Symbol::new(rodetc_span::symbol::PREDEFINED_SYMBOLS_COUNT + ${index()});
        )*
    };
}

// List of extra symbols to be included in Miri.
// An alternative content can be specified using a colon after the symbol name.
generate! {
    sys_mutex_lock,
    sys_mutex_try_lock,
    sys_mutex_unlock,
}
