/// The responsibility of my lib.rs file is to control my public API.

#[cfg(test)]
mod unit_tests;

pub mod parsers {
    pub mod first;
    pub mod mealy;
    pub mod moore;
    #[cfg(feature = "nom")]
    pub mod nom;
    #[cfg(feature = "pest")]
    pub mod pest;
    pub mod split;
    pub mod state;
}

pub use crate::parsers::first;
pub use crate::parsers::mealy;
pub use crate::parsers::moore;
#[cfg(feature = "nom")]
pub use crate::parsers::nom;
#[cfg(feature = "pest")]
pub use crate::parsers::pest;
pub use crate::parsers::split;
pub use crate::parsers::state;

mod data_model;
pub use crate::data_model::Error;
pub use crate::data_model::ErrorKind;
pub use crate::data_model::Wsv;
pub use crate::data_model::WsvValue;

pub mod io;
pub use crate::io::parse_from_buf_file;
pub use crate::io::parse_from_memory;
pub use crate::io::parse_from_whole_file;

pub use crate::state::parse_line;
