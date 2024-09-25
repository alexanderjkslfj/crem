//! Calculate with minimal precision loss: Terms created using crem are automatically simplified, reducing precision loss to a minimum.
//!
//! ### Directly process a term using basic numbers
//! ```rust
//! # use crem::*;
//! let inaccurate: f64 = 0.1 + 0.2;
//! assert_ne!(inaccurate, 0.3);
//!
//! let accurate: f64 = Term::process("0.1 + 0.2")?;
//! assert_eq!(accurate, 0.3);
//! # Ok::<(), TryFromStrError>(())
//! ```
//! ```rust
//! # use crem::*;
//! let rounded: i64 = 2 / 3 * 3;
//! assert_ne!(rounded, 2);
//!
//! let precise: i64 = Term::process("2 / 3 * 3")?;
//! assert_eq!(precise, 2);
//! # Ok::<(), TryFromStrError>(())
//! ```
//!
//! ### Work with foreign number types
//! ```rust
//! # use crem::Term;
//! use num_bigint::BigInt;
//!
//! let term =
//!     Term::from(BigInt::from(1)) / Term::from(BigInt::from(7)) * Term::from(BigInt::from(7));
//!
//! let result: BigInt = term.calc();
//!
//! assert_eq!(result, BigInt::from(1));
//! ```
//!
//! ### Prepare terms using variables
//! ```rust
//! # use crem::Term;
//! let feet = Term::from(3.28084) * Term::var("meters");
//!
//! let two_meters_in_feet: f64 = feet.with_var("meters", &Term::from(2.0)).calc();
//!
//! assert_eq!(two_meters_in_feet, 6.56168);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod operation;
mod parse_string;
mod term;

pub use parse_string::TryFromStrError;
pub use term::Term;
