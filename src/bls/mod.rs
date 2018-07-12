// BLS Signatures.
// From: "Dan Boneh, Manu Drijvers, Gregory Neven. Compact Multi-Signatures for Smaller Blockchains".
// Available from: https://eprint.iacr.org/2018/483.pdf
// This link was helpful too https://crypto.stanford.edu/~dabo/pubs/papers/BLSmultisig.html

// TODO: Add domain separation, for single sig, aggregation sig
// TODO: Create a common trait that has to_bytes and from_bytes that can be used by the various structs
// TODO: Support point compression

extern crate amcl;
extern crate rand;

pub mod common;
pub mod simple;
pub mod aggr_new;
pub mod aggr_old;
pub mod types;
pub mod constants;
mod amcl_utils;

