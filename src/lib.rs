#![doc = include_str!("../README.md")] // This is the landing page of cargo doc, so make it the same as the codebase.

// The responsibility of my lib.rs file is to control my public API.
#[cfg(test)]
mod unit_tests;

#[doc(inline)]
pub use first::parse_line;

mod data_model;
pub use data_model::*; // This does not override the pub(crate) declaration of the Parser enum, nor does it throw an error. The glob only takes the pub items, Meaning you can safely use globs in re-exports, since the item will be accessible anyway.
//pub use crate::data_model::Parser; //trying to expose it explicitly causes an error, as expected.


// the revelation here is that I can comfortably use glob re-exports, meaning I can define upfront the access to modules.
// Does the glob re-export work for modules too?
mod io;
#[doc(inline)]
pub use io::*;

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
    //pub mod typestate;
}

// looks like globs work for modules too! This bit of code allows you to control the hierarchy very easily.
//#[doc(inline)] // does this work on a glob?
pub use parsers::*; // somehow this works even though the module is not declared as pub(super). Probably because the mod is in this mod, so it is private to lib.rs.

// The following flattens the parsers folder/module, so from the outside it looks like all of these are
// #[doc(inline)]
// pub use crate::parsers::first;
// #[doc(inline)]
// pub use crate::parsers::mealy;
// #[doc(inline)]
// pub use crate::parsers::moore;
#[cfg(feature = "nom")] //how does the glob reex affect feature flags?
//#[doc(inline)]
pub use parsers::nom; // seems you can do it twice anyway with no redundancy error. Possibly good for feature flags. Does the doc attr apply either way, or do the specific attributes take precidence?
// #[doc(inline)]
// #[cfg(feature = "pest")]
// pub use crate::parsers::pest;
// #[doc(inline)]
// pub use crate::parsers::regex;
// #[doc(inline)]
// pub use crate::parsers::split;
// #[doc(inline)]
// pub use crate::parsers::state;
// #[doc(inline)]
// pub use crate::parsers::typestate;
