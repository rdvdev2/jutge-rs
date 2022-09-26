//! A library crate for easy and idiomatic interaction with
//! <https://jutge.org>

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

macro_rules! modules {
    ($($mod:ident),+; $($feature:literal => $($f_mod:ident),+);*;) => {
        $(
            mod $mod;
            pub use $mod::*;
            )+
        $(
            $(
                #[cfg(feature=$feature)]
                mod $f_mod;
                #[cfg(feature=$feature)]
                pub use $f_mod::*;
                )+
            )*
    };
}

modules! {
    error, problem;
    "web-client" => client;
}
