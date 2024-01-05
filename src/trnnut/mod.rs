// Copyright 2022-2023 Futureverse Corporation Limited
//! # TRNNut
//!
//! Collection of versioned `TRNNuts`
//!

use alloc::fmt::{self, Display, Formatter};
use alloc::string::String;

use codec::{Decode, Encode, Input, Output};
use pact::interpreter::types::PactType;

use crate::PartialDecode;
use crate::ValidationErr;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
pub mod v0;

use core::convert::TryFrom;
use v0::TRNNutV0;
use TRNNut::V0;

pub type ModuleName = String;
pub type MethodName = String;
pub type ContractAddress = [u8; 32];
pub const CONTRACT_WILDCARD: ContractAddress = [0_u8; 32];
pub const WILDCARD: &str = "*";

/// A TRN module permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeDomain {
    Method,
    MethodArguments,
    Module,
}

/// A TRN contract permission domain
#[derive(Debug, Eq, PartialEq)]
pub enum ContractDomain {
    Contract,
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

impl Display for ContractDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract => write!(f, "contract"),
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
    fn encode_to<T: Output>(&self, buf: &mut T) {
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

    /// Validates a TRNNut smart contract call by:
    /// (1) identifying the version to be validated
    /// (2) executing the specific trnnut version's validation function
    ///
    /// # Errors
    ///
    /// Will return error if validation fails with the type of error embedded in `ContractDomain`
    pub fn validate_contract_call(
        &self,
        contract_address: &ContractAddress,
    ) -> Result<(), ValidationErr<ContractDomain>> {
        match &self {
            V0(inner) => inner.validate_contract(*contract_address),
        }
    }
}

#[cfg(test)]
mod test {
    use super::v0::{contract::Contract, method::Method, module::Module};
    use super::{TRNNut, TRNNutV0, ContractAddress, MethodName, ModuleName};

    fn make_methods(method: &Method) -> Vec<(MethodName, Method)> {
        let mut methods = Vec::<(MethodName, Method)>::default();
        methods.push((method.name.clone(), method.clone()));
        methods
    }

    fn make_modules(module: &Module) -> Vec<(ModuleName, Module)> {
        let mut modules = Vec::<(ModuleName, Module)>::default();
        modules.push((module.name.clone(), module.clone()));
        modules
    }

    fn make_contracts(contract: &Contract) -> Vec<(ContractAddress, Contract)> {
        let mut contracts = Vec::<(ContractAddress, Contract)>::default();
        contracts.push((contract.address, contract.clone()));
        contracts
    }

    #[test]
    fn it_validates_v0_module() {
        let method = Method::new("*");
        let methods = make_methods(&method);
        let module = Module::new("module_test").methods(methods);
        let modules = make_modules(&module);

        let contracts = Vec::<(ContractAddress, Contract)>::default();

        let trnnut = TRNNut::V0(TRNNutV0 { modules, contracts });

        assert_eq!(
            trnnut.validate_runtime_call(&module.name, &method.name, &[]),
            Ok(())
        );
    }

    #[test]
    fn it_validates_v0_contracts() {
        let modules = Vec::<(ModuleName, Module)>::default();

        let contract = Contract::new(&[0x12_u8; 32]);
        let contracts = make_contracts(&contract);

        let trnnut = TRNNut::V0(TRNNutV0 { modules, contracts });

        assert_eq!(trnnut.validate_contract_call(&contract.address), Ok(()));
    }
}
