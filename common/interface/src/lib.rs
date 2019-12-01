#![cfg_attr(feature = "wasm", no_std)]

pub mod address;
pub mod raw_transaction;

pub use address::Address;
pub use raw_transaction::RawTransaction;

#[cfg(not(feature = "wasm"))]
mod contract;

#[cfg(not(feature = "wasm"))]
pub use contract::Contract;

#[cfg(not(feature = "wasm"))]
pub mod transaction;

#[cfg(not(feature = "wasm"))]
pub use transaction::Transaction;
