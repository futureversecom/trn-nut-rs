// Copyright 2022-2023 Futureverse Corporation Limited

//! Provide JS-Rust API bindings to create and inspect TRNNut
use trnnut_rs::{
    v0::{contract::Contract, module::Module, TRNNutV0},
    TRNNut,
};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[inline]
fn from_slice_32(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    if bytes.len() < 32 {
        log("expected 32 byte array");
        return array;
    }
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

/// A js handle for a rust versioned trnnut struct
#[wasm_bindgen(js_name = TRNNut)]
pub struct JsHandle(TRNNut);

#[wasm_bindgen(js_class = TRNNut)]
#[allow(irrefutable_let_patterns)]
impl JsHandle {
    #[wasm_bindgen(constructor)]
    /// Create a new TRNNut, it is always v0 for now
    pub fn new(modules: &JsValue, contracts: &JsValue) -> Self {
        let modules_vec: Vec<(String, Module)> = modules
            .into_serde()
            .expect("Deserialization of modules failed");
        let contract_vec: Vec<([u8; 32], Contract)> = contracts
            .into_serde()
            .expect("Deserialization of contracts failed");
        let trnnut: TRNNutV0 = TRNNutV0 {
            modules: modules_vec,
            contracts: contract_vec,
        };
        JsHandle(TRNNut::V0(trnnut))
    }

    #[allow(non_snake_case)]
    /// Return the trnnut module
    pub fn getModule(&self, module: &str) -> JsValue {
        if let TRNNut::V0(trnnut) = &self.0 {
            if trnnut.get_module(module).is_none() {
                return JsValue::UNDEFINED;
            }
            return JsValue::from_serde(&trnnut.get_module(module).unwrap()).unwrap();
        }
        panic!("unsupported trnnut version");
    }

    #[allow(non_snake_case)]
    /// Return the trnnut contract
    pub fn getContract(&self, contract_address: &[u8]) -> JsValue {
        if let TRNNut::V0(trnnut) = &self.0 {
            if trnnut
                .get_contract(from_slice_32(contract_address))
                .is_none()
            {
                return JsValue::UNDEFINED;
            }
            return JsValue::from_serde(
                &trnnut
                    .get_contract(from_slice_32(contract_address))
                    .unwrap(),
            )
            .unwrap();
        }
        panic!("unsupported trnnut version");
    }

    #[allow(non_snake_case)]
    /// Verify trnnut is valid for contract_address
    pub fn verifyContract(&self, contract_address: &[u8]) -> bool {
        if let TRNNut::V0(trnnut) = &self.0 {
            return trnnut
                .validate_contract(from_slice_32(contract_address))
                .is_ok();
        }
        panic!("unsupported trnnut version");
    }

    /// Encode the trnnut into bytes
    pub fn encode(&mut self) -> Vec<u8> {
        self.0.encode()
    }

    /// Decode a version 0 trnnut from `input` bytes
    pub fn decode(input: &[u8]) -> Result<JsHandle, JsValue> {
        match TRNNut::decode(&mut &input[..]) {
            Ok(trnnut) => Ok(JsHandle(trnnut)),
            Err(err) => {
                log(&format!("failed decoding: {:?}", err));
                Err(JsValue::undefined())
            }
        }
    }
}
