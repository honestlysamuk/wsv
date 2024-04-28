mod data_model;
pub use crate::data_model::Wsv;
pub use crate::data_model::WsvError;
pub use crate::data_model::WsvValue;

mod parsers;

#[cfg(feature = "nom")]
pub use crate::parsers::nom;
#[cfg(feature = "pest")]
pub use crate::parsers::pest;

pub use crate::parsers::primitive;
pub use crate::parsers::split;

pub use crate::parsers::primitive::parse;

impl TryFrom<&str> for Wsv {
    type Error = WsvError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        parse(input)
    }
}

impl TryFrom<String> for Wsv {
    type Error = WsvError;
    fn try_from(input: String) -> Result<Self, Self::Error> {
        parse(&input)
    }
}
