#![cfg_attr(feature = "wasm", no_std)]

pub mod address;
pub use address::Address;

#[cfg(not(feature = "wasm"))]
mod contract;

#[cfg(not(feature = "wasm"))]
pub use contract::Contract;
