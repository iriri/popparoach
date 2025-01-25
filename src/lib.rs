//! # Examples
//! ```rust,ignore
//! # use popparoach::Cut;
//! #
//! # fn lol() {
//! let xs = [0, 1, 2, 3];
//! assert_eq!(xs.cut_at::<2>(), (xs.cut::<0, 2>(), xs.cut::<2, 4>()));
//! # }
//! ```
#![no_std]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

pub mod compat;

#[cfg(feature = "nightly")]
mod nightly;

#[cfg(feature = "nightly")]
pub use nightly::{Cut, CutAt};
