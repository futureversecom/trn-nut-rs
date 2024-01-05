// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! # TRNNut
//!
//! Delegated authority nut for TRN
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

use codec::Input;
pub use core::convert::TryFrom;

mod trnnut;
mod validation;

pub use crate::trnnut::ContractDomain;
pub use crate::trnnut::RuntimeDomain;

pub use crate::trnnut::v0;

pub use crate::trnnut::v0::TRNNutV0;
pub use crate::trnnut::TRNNut;
pub use crate::validation::ValidationErr;

mod test;

pub trait PartialDecode: Sized {
    /// decode an input which is not including the version as the up front two bytes
    ///
    /// # Errors
    ///
    /// On failure, returns a `codec::Error`
    fn partial_decode<I: Input>(input: &mut I) -> Result<Self, codec::Error>;
}
