#![cfg_attr(feature = "wasm", no_std)]

pub mod address;
pub mod transaction;

pub use address::Address;
pub use transaction::Transaction;

#[cfg(not(feature = "wasm"))]
mod contract;

#[cfg(not(feature = "wasm"))]
pub use contract::Contract;
