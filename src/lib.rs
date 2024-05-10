mod data_model;
pub use crate::data_model::Wsv;
pub use crate::data_model::Error;
pub use crate::data_model::WsvValue;

mod parsers;

#[cfg(test)]
mod unit_tests;


#[cfg(feature = "nom")]
pub use crate::parsers::nom;
#[cfg(feature = "pest")]
pub use crate::parsers::pest;
pub use crate::parsers::first;
pub use crate::parsers::split;
pub use crate::parsers::state;

pub use crate::parsers::state::parse;