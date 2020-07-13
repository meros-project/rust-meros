use std::fmt;

pub mod core;
pub mod crypto;
pub mod db;
pub mod primitives;

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

/// The trait that enables serialization.
pub trait CanSerialize {
    fn to_bytes(&self) -> bincode::Result<Vec<u8>>;
    fn from_bytes<T>(bytes: Vec<u8>) -> bincode::Result<T>
    where
        T: CanSerialize;

    // fn from_bytes(bytes: Vec<u8>) -> Self;
}
