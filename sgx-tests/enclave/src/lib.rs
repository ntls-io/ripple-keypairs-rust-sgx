#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

use std::backtrace;
use std::string::String;
use std::vec::Vec;

use sgx_tunittest::*;

#[path = "../../../tests/api.rs"]
mod api;

#[no_mangle]
pub extern "C" fn run_tests_ecall() -> usize {
    backtrace::enable_backtrace("enclave.signed.so", backtrace::PrintFormat::Short).unwrap();

    rsgx_unit_tests!(
        api::secp256k1::new_seed,
        api::secp256k1::random_seed_starts_with_s,
        api::secp256k1::random_seed,
        api::secp256k1::parse_random_seed,
        api::secp256k1::parse_seed,
        api::secp256k1::bad_seed,
        api::secp256k1::derive_keypair,
        api::secp256k1::sign,
        api::secp256k1::verify,
        api::secp256k1::verify_bad_signature,
        api::secp256k1::derive_address,
        api::secp256k1::random_address,
        api::ed25519::new_seed,
        api::ed25519::random_seed_starts_with_sed,
        api::ed25519::random_seed,
        api::ed25519::parse_random_seed,
        api::ed25519::parse_seed,
        api::ed25519::bad_seed,
        api::ed25519::derive_keypair,
        api::ed25519::sign,
        api::ed25519::verify,
        api::ed25519::verify_bad_signature,
        api::ed25519::derive_address,
        api::ed25519::random_address,
    )
}
