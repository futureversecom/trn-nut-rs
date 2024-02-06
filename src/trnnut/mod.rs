// Copyright 2022-2023 Futureverse Corporation Limited
//! # TRNNut
//!
//! Collection of versioned `TRNNuts`
//!

use alloc::fmt::{self, Display, Formatter};
use codec::{Decode, Encode, Input, Output};
use pact::types::PactType;

use crate::PartialDecode;
use crate::ValidationErr;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
pub mod v0;

use core::convert::TryFrom;
use v0::TRNNutV0;
use TRNNut::V0;

pub const WILDCARD: &str = "*";

/// A TRN module permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeDomain {
    Method,
    MethodArguments,
    Module,
}

impl Display for RuntimeDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Method => write!(f, "method"),
            Self::MethodArguments => write!(f, "method arguments"),
            Self::Module => write!(f, "module"),
        }
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(Clone, Debug, Eq, PartialEq))]
pub enum TRNNut {
    V0(TRNNutV0),
}

#[allow(unreachable_patterns)]
impl TryFrom<TRNNut> for TRNNutV0 {
    type Error = codec::Error;
    fn try_from(v: TRNNut) -> Result<Self, Self::Error> {
        match v {
            V0(inner) => Ok(inner),
            _ => Err(codec::Error::from("TRNNut version is not 0")),
        }
    }
}

impl Encode for TRNNut {
    fn encode_to<T: Output + ?Sized>(&self, buf: &mut T) {
        match &self {
            V0(inner) => inner.encode_to(buf),
        }
    }
}

impl Decode for TRNNut {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        let version = u16::from_le_bytes([input.read_byte()?, input.read_byte()?]);

        match version {
            0 => match TRNNutV0::partial_decode(input) {
                Ok(inner) => Ok(V0(inner)),
                Err(e) => Err(e),
            },
            _ => Err(codec::Error::from("unexpected version")),
        }
    }
}

impl TRNNut {
    /// Validates a TRNNut runtime module call by:
    /// (1) identifying the version to be validated
    /// (2) executing the specific trnnut version's validation function
    ///
    /// # Errors
    ///
    /// Will return error if validation fails with the type of error embedded in `RuntimeDomain`
    pub fn validate_runtime_call(
        &self,
        module_name: &str,
        method_name: &str,
        args: &[PactType],
    ) -> Result<(), ValidationErr<RuntimeDomain>> {
        match &self {
            V0(inner) => inner.validate_module(module_name, method_name, args),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::v0::{method::Method, module::Module};

    fn make_methods(method: &Method) -> Vec<Method> {
        let mut methods = Vec::<Method>::default();
        methods.push(method.clone());
        methods
    }

    fn make_modules(module: &Module) -> Vec<Module> {
        let mut modules = Vec::<Module>::default();
        modules.push(module.clone());
        modules
    }

    #[test]
    fn it_validates_v0_module() {
        let method = Method::new("*");
        let methods = make_methods(&method);
        let module = Module::new("module_test").methods(methods);
        let modules = make_modules(&module);

        let trnnut = TRNNut::V0(TRNNutV0 { modules });

        assert_eq!(
            trnnut.validate_runtime_call(&module.name, &method.name, &[]),
            Ok(())
        );
    }
}
