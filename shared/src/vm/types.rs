//! VM types can be passed between the host and guest via wasm linear
//! memory.
//!
//! These are either:
//! 1. Module call types
//!    The module call inputs are passed host-to-guest.
//!
//! 2. Execution environment types
//!    The environment inputs are passed guest-to-host and outputs back from
//!    host-to-guest.

use std::collections::HashSet;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{Address, Key};

/// Input for transaction wasm module call
pub type TxInput = Vec<u8>;

/// Input for validity predicate wasm module call
pub struct VpInput<'a> {
    pub addr: &'a Address,
    pub data: &'a [u8],
    /// The storage changed keys from the write log of storage updates
    /// performed by the transaction for the account associated with the VP
    pub keys_changed: &'a [Key],
    /// The verifiers to trigger VPs
    pub verifiers: &'a HashSet<Address>,
}

/// Input for matchmaker wasm module call
pub type MatchmakerInput = Vec<u8>;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct KeyVal {
    pub key: String,
    pub val: Vec<u8>,
}