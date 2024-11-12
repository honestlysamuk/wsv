#![doc = include_str!("../README.md")]

// The responsibility of my lib.rs file is to control my public API.
#[cfg(test)]
mod unit_tests;

mod data_model;
pub use crate::data_model::Error;
pub use crate::data_model::ErrorKind;
pub use crate::data_model::Wsv;
pub use crate::data_model::WsvValue;

mod io;
#[doc(inline)]
pub use crate::first::parse_line;
#[doc(inline)]
pub use crate::io::from_reader;
#[doc(inline)]
pub use crate::io::from_string;
mod parsers {
    pub mod first;
    pub mod mealy;
    pub mod moore;
    #[cfg(feature = "nom")]
    pub mod nom;
    #[cfg(feature = "pest")]
    pub mod pest;
    pub mod regex;
    pub mod split;
    pub mod state;
    pub mod typestate;
}
#[doc(inline)]
pub use crate::parsers::first;
#[doc(inline)]
pub use crate::parsers::mealy;
#[doc(inline)]
pub use crate::parsers::moore;
#[cfg(feature = "nom")]
#[doc(inline)]
pub use crate::parsers::nom;
#[doc(inline)]
#[cfg(feature = "pest")]
pub use crate::parsers::pest;
#[doc(inline)]
pub use crate::parsers::regex;
#[doc(inline)]
pub use crate::parsers::split;
#[doc(inline)]
pub use crate::parsers::state;
#[doc(inline)]
pub use crate::parsers::typestate;
