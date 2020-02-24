// Copyright (C) 2019-2020 Centrality Investments Limited
//!
//! CENNZnut - Integration Tests
//!

#![warn(clippy::pedantic)]
#![cfg(test)]

use super::method::Method;
use super::module::Module;
use super::WILDCARD;
use crate::cennznut::ModuleDomain;
use crate::{CENNZnut, CENNZnutV0, TryFrom, Validate, ValidationErr};

use bit_reverse::ParallelReverse;
use codec::{Decode, Encode};
use pact::contract::{Contract as PactContract, DataTable};
use pact::interpreter::OpCode;
use pact::types::{Numeric, PactType, StringLike};
use std::string::String;
use std::vec::Vec;

fn make_methods(method: &Method) -> Vec<(String, Method)> {
    let mut methods: Vec<(String, Method)> = Vec::default();
    methods.push((method.name.clone(), method.clone()));
    methods
}

fn make_modules(module: &Module) -> Vec<(String, Module)> {
    let mut modules: Vec<(String, Module)> = Vec::default();
    modules.push((module.name.clone(), module.clone()));
    modules
}

#[test]
fn it_works_encode() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let encoded = cennznut.encode();

    assert_eq!(
        encoded,
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    assert_eq!(encoded[2], 0); // 1 module encodes to 0
}

#[test]
fn it_works_encode_one_module() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");
    assert_eq!(c.encode(), encoded);
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(c0.modules.len(), 1);
}

#[test]
fn it_works_encode_with_module_cooldown() {
    let method = Method::new("method_test");
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 0, 109, 101, 116, 104,
            111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode_with_module_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 0, 109, 101, 116, 104, 111, 100, 95,
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("It works");
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .block_cooldown,
        Some(86_400)
    );
}

#[test]
fn it_works_encode_with_method_cooldown() {
    let method = Method::new("method_test").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    assert_eq!(
        cennznut.encode(),
        vec![
            0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104,
            111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 222, 0, 0, 0
        ]
    );
}

#[test]
fn it_works_decode_with_method_cooldown() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 192, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111, 100,
        95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222,
        0, 0, 0,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("It works");
    let c0 = CENNZnutV0::try_from(c).unwrap();
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .block_cooldown,
        Some(86_400)
    );
    assert_eq!(
        c0.get_module("module_test")
            .expect("module exists")
            .get_method("method_test")
            .expect("method exists")
            .block_cooldown,
        Some(123)
    );
}

#[test]
fn it_works_decode_with_version_0() {
    let encoded: Vec<u8> = vec![1, 2, 3, 192];
    assert_eq!(
        CENNZnutV0::decode(&mut &encoded[..]),
        Err(codec::Error::from("expected version : 0"))
    );
}

#[test]
fn it_works_encode_with_constraints() {
    let contract = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(111)),
            PactType::Numeric(Numeric(333)),
            PactType::StringLike(StringLike(b"testing")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test").constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test").methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let encoded = cennznut.encode();

    assert_eq!(
        encoded,
        vec![
            0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116,
            101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
            192, 128, 16, 246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224,
            116, 101, 115, 116, 105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
        ]
    );
    let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
    #[allow(clippy::cast_possible_truncation)]
    let len_byte = constraints.len() as u8;
    assert_eq!(
        encoded[constraints_length_byte_cursor],
        (len_byte - 1).swap_bits()
    );
}

#[test]
fn it_works_decode_with_constraints() {
    let encoded: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
    ];
    let c: CENNZnut = Decode::decode(&mut &encoded[..]).expect("it works");
    assert_eq!(c.encode(), encoded);

    let c0 = CENNZnutV0::try_from(c).unwrap();
    let method = &c0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists");

    if let Some(constraints) = &method.constraints {
        let constraints_length_byte_cursor: usize = 4 + 32 + 1 + 32;
        #[allow(clippy::cast_possible_truncation)]
        let len_byte = constraints.len() as u8;
        assert_eq!(
            encoded[constraints_length_byte_cursor].swap_bits() + 1,
            len_byte,
        );
    };
}

#[test]
fn it_works_with_lots_of_things_codec() {
    let method = Method::new("method_test").block_cooldown(123);
    let method2 = Method::new("method_test2").block_cooldown(321);

    let mut methods: Vec<(String, Method)> = Vec::default();
    methods.push((method.name.clone(), method.clone()));
    methods.push((method2.name.clone(), method2.clone()));

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods.clone());
    let module2 = Module::new("module_test2")
        .block_cooldown(55_555)
        .methods(methods.clone());

    let mut modules: Vec<(String, Module)> = Vec::default();
    modules.push((module.name.clone(), module.clone()));
    modules.push((module2.name.clone(), module2.clone()));

    let cennznut = CENNZnutV0 { modules };

    let encoded = vec![
        0, 0, 128, 160, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 138, 128, 0, 128, 109, 101, 116, 104, 111,
        100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        222, 0, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 128, 0, 0, 160, 109, 111, 100, 117, 108,
        101, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 192, 155, 0, 0, 128, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115, 116, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0, 128, 109, 101, 116, 104,
        111, 100, 95, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 130, 128, 0, 0,
    ];
    assert_eq!(cennznut.encode(), encoded);
    assert_eq!(cennznut, CENNZnutV0::decode(&mut &encoded[..]).unwrap());
}

#[test]
fn it_validates() {
    let contract = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"test")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(123)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(cennznut.validate(&module.name, &method.name, &args), Ok(()));
    assert_eq!(
        cennznut.validate("module_test2", &method.name, &args),
        Err(ValidationErr::NoPermission(ModuleDomain::Module))
    );
    assert_eq!(
        cennznut.validate(&module.name, "method_test2", &args),
        Err(ValidationErr::NoPermission(ModuleDomain::Method))
    );
}

#[test]
fn it_validates_error_with_bad_bytecode() {
    let contract = PactContract {
        data_table: DataTable::new(vec![PactType::StringLike(StringLike(b"test"))]),
        bytecode: [OpCode::GT.into(), 0, 0, 1, 0].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [PactType::StringLike(StringLike(b"test"))];

    assert_eq!(
        cennznut.validate(&module.name, &method.name, &args),
        Err(ValidationErr::ConstraintsInterpretation)
    );
}

#[test]
fn it_validates_error_with_false_constraints() {
    let contract = PactContract {
        data_table: DataTable::new(vec![
            PactType::Numeric(Numeric(123)),
            PactType::StringLike(StringLike(b"a")),
        ]),
        bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
    };
    let mut constraints: Vec<u8> = Vec::new();
    contract.encode(&mut constraints);

    let method = Method::new("method_test")
        .block_cooldown(123)
        .constraints(constraints.clone());
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(321)),
        PactType::StringLike(StringLike(b"b")),
    ];

    assert_eq!(
        cennznut.validate(&module.name, &method.name, &args),
        Err(ValidationErr::NoPermission(ModuleDomain::MethodArguments))
    );
}

#[test]
fn it_validates_with_empty_constraints() {
    let method = Method::new("method_test").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(86_400)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [
        PactType::Numeric(Numeric(0)),
        PactType::StringLike(StringLike(b"test")),
    ];

    assert_eq!(cennznut.validate(&module.name, &method.name, &args), Ok(()));
}

#[test]
fn it_works_get_pact() {
    // A CENNZnut with constraints set
    let encoded_with: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 192, 128, 16,
        246, 0, 0, 0, 0, 0, 0, 0, 128, 16, 178, 128, 0, 0, 0, 0, 0, 0, 0, 224, 116, 101, 115, 116,
        105, 110, 103, 5, 0, 0, 1, 0, 5, 0, 1, 1, 1,
    ];

    let cennznut_with: CENNZnut = Decode::decode(&mut &encoded_with[..]).expect("it works");
    let cennznut_with_v0 = CENNZnutV0::try_from(cennznut_with).unwrap();
    let contract_with = cennznut_with_v0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    if let Some(contract) = contract_with {
        assert_eq!(
            contract,
            PactContract {
                data_table: DataTable::new(vec![
                    PactType::Numeric(Numeric(111)),
                    PactType::Numeric(Numeric(333)),
                    PactType::StringLike(StringLike(b"testing")),
                ]),
                bytecode: [OpCode::EQ.into(), 0, 0, 1, 0, OpCode::EQ.into(), 0, 1, 1, 1].to_vec(),
            }
        );
    }

    // A CENNZnut without constraints set
    let encoded_without: Vec<u8> = vec![
        0, 0, 0, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let cennznut_without: CENNZnut = Decode::decode(&mut &encoded_without[..]).expect("it works");
    let cennznut_without_v0 = CENNZnutV0::try_from(cennznut_without).unwrap();
    let contract_without = cennznut_without_v0
        .get_module("module_test")
        .expect("module exists")
        .get_method("method_test")
        .expect("method exists")
        .get_pact();

    assert_eq!(contract_without, None);
}

#[test]
fn wildcard_method() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);

    let result = module.get_method("my_unregistered_method");
    assert_ne!(result, None);
}

#[test]
fn wildcard_method_validates() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [];

    assert_eq!(
        cennznut.validate(&module.name, "my_unregistered_method", &args),
        Ok(())
    );
}

#[test]
fn wildcard_module() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };

    let result = cennznut.get_module("my_unregistered_module");
    assert_ne!(result, None);
}

#[test]
fn wildcard_module_validates() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [];

    assert_eq!(
        cennznut.validate("my_unregistered_module", "registered_method", &args),
        Ok(())
    );
}

#[test]
fn wildcard_module_wildcard_method_validates() {
    let method = Method::new(WILDCARD).block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new(WILDCARD).block_cooldown(1).methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [];

    assert_eq!(
        cennznut.validate("my_unregistered_module", "my_unregistered_method", &args),
        Ok(())
    );
}

#[test]
fn unregistered_module_fails_validation() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("registered_module")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [];

    assert_eq!(
        cennznut.validate("my_unregistered_module", "registered_method", &args),
        Err(ValidationErr::NoPermission(ModuleDomain::Module))
    );
}

#[test]
fn unregistered_method_fails_validation() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let module = Module::new("registered_module")
        .block_cooldown(1)
        .methods(methods);
    let modules = make_modules(&module);

    let cennznut = CENNZnutV0 { modules };
    let args = [];

    assert_eq!(
        cennznut.validate("registered_module", "my_unregistered_method", &args),
        Err(ValidationErr::NoPermission(ModuleDomain::Method))
    );
}

#[test]
fn registered_methods_have_priority_over_wildcard_methods() {
    let wild_method = Method::new(WILDCARD).block_cooldown(123);
    let registered_method = Method::new("registered_method").block_cooldown(123);

    let mut methods: Vec<(String, Method)> = Vec::default();
    methods.push((wild_method.name.clone(), wild_method.clone()));
    methods.push((registered_method.name.clone(), registered_method.clone()));

    let module = Module::new("module_test")
        .block_cooldown(1)
        .methods(methods);

    let result = module.get_method("registered_method").unwrap();

    assert_eq!(result.name, "registered_method");
}

#[test]
fn registered_modules_have_priority_over_wildcard_modules() {
    let method = Method::new("registered_method").block_cooldown(123);
    let methods = make_methods(&method);

    let wild_module = Module::new(WILDCARD)
        .block_cooldown(123)
        .methods(methods.clone());
    let registered_module = Module::new("registered_module")
        .block_cooldown(123)
        .methods(methods);

    let mut modules: Vec<(String, Module)> = Vec::default();
    modules.push((wild_module.name.clone(), wild_module.clone()));
    modules.push((registered_module.name.clone(), registered_module.clone()));

    let cennznut = CENNZnutV0 { modules };

    let result = cennznut.get_module("registered_module").unwrap();

    assert_eq!(result.name, "registered_module");
}
