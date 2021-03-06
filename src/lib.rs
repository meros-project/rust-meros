pub mod common;
pub mod crypto;
pub mod p2p;
pub mod primitives;

use std::fmt;

#[derive(Debug)]
pub struct GeneralError {
    details: String,
}

impl GeneralError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for GeneralError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// The trait that enables serialization and deserialization.
pub trait CanSerialize {
    type S;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>>;
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self::S>;
}

/// Add compression and decompression functionality to a serializable type.
pub trait Compressable {
    fn compress(&self) -> Vec<u8>;
    fn decompress(bytes: Vec<u8>) -> Self;
}
