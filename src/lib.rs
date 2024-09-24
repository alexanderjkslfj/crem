//! Calculate with minimal precision loss: Terms created using `crem` are automatically simplified, reducing precision loss to a minimum.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod operation;
mod parse_string;
mod term;

pub use parse_string::TryFromStrError;
pub use term::Term;
