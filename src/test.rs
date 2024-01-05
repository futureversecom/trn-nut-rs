// Copyright 2022-2023 Futureverse Corporation Limited
//!
//! TRNNut - Integration Tests
//!

#![warn(clippy::pedantic)]
#![cfg(test)]

use crate::TRNNut;

use codec::Decode;
use std::vec::Vec;

#[test]
fn it_fails_decode_with_invalid_constraints() {
    let encoded_trnnut: Vec<u8> = vec![
        0, 0, 1, 64, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 109, 101, 116, 104, 111, 100, 95, 116, 101, 115,
        116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let bad_type_id: Vec<u8> = vec![3, 0, 0b1000_0000, 0b0000_0001, 0b0000_0001];
    let n_too_short: Vec<u8> = vec![1, 0, 1];
    let n_too_large: Vec<u8> = vec![3, 0, 0b1000_0000, 0b1000_0000, 0b0000_1111];

    let encoded_with_bad_type_id: Vec<u8> = [encoded_trnnut.clone(), bad_type_id].concat();
    let encoded_with_n_too_short: Vec<u8> = [encoded_trnnut.clone(), n_too_short].concat();
    let encoded_with_n_too_large: Vec<u8> = [encoded_trnnut, n_too_large].concat();

    assert_eq!(
        TRNNut::decode(&mut &encoded_with_bad_type_id[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        TRNNut::decode(&mut &encoded_with_n_too_short[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
    assert_eq!(
        TRNNut::decode(&mut &encoded_with_n_too_large[..]),
        Err(codec::Error::from("invalid constraints codec")),
    );
}
