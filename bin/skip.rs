//! To build the binary:
//!
//!     `cargo build --release --bin skip`
//!
//! To build the circuit:
//!
//!     `./target/release/circuit_function_field build`
//!
//! To prove the circuit using evm io:
//!
//!    `./target/release/circuit_function_evm prove --input-json src/bin/circuit_function_evm_input.json`
//!
//! Note that this circuit will not work with field-based io.
//!
//!
//!
//!
use plonky2x::backend::function::Plonky2xFunction;
use tendermintx::config::{BanksyConfig, BANKSY_TESTNET_CHAIN_ID_SIZE_BYTES};
use tendermintx::consts::VALIDATOR_SET_SIZE_MAX;
use tendermintx::skip::SkipCircuit;

fn main() {
    // Note: Defaults to using the CelestiaConfig, but any Tendermint chain config can be used.
    SkipCircuit::<VALIDATOR_SET_SIZE_MAX, BANKSY_TESTNET_CHAIN_ID_SIZE_BYTES, BanksyConfig>::entrypoint(
    );
}
