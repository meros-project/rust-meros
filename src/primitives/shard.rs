use crate::{
    core::Compressable,
    crypto::hash,
    db::{IsKey, IsValue},
    GeneralError,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

/// All of the errors that a `Shard` method could throw.
#[derive(Debug)]
pub enum ShardError {
    SerializeError(bincode::Error),
    TimestampError(SystemTimeError),
    InvalidSplitSizes(GeneralError),
}

/// The structure used for the identification of a shard on the meros
/// network.
#[derive(Serialize, Deserialize, Debug)]
pub struct ShardID {
    id: hash::Hash,
}

impl ShardID {
    pub fn new(data: &Vec<u8>) -> Result<(Self, u128), ShardError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ShardError::TimestampError(e))?
            .as_secs() as u128;

        let data =
            [&data[..], time.to_string().as_bytes()].concat().to_vec();
        Ok((
            Self {
                id: hash::hash_bytes(data),
            },
            time,
        ))
    }
}

impl PartialEq for ShardID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl IsKey for ShardID {}

/// The structure representing a `Shard` to be stored in a node's
/// local shard database.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub data: Vec<u8>,
    size: usize,
    timestamp: u128,
    index: u32,

    id: ShardID,
}

impl Shard {
    pub fn new(data: Vec<u8>, index: u32) -> Result<Shard, ShardError> {
        let (id, timestamp) = ShardID::new(&data)
            .map_err(|e| ShardError::ShardIDError(e))?;

        Ok(Shard {
            size: data.len(),
            data,
            timestamp,
            index,
            id,
        })
    }
}

/// Split a vector of bytes as described by the `sizes` parameter and
/// return properly distributed `Shard`s.
pub fn split_bytes(
    bytes: &Vec<u8>,
    sizes: &Vec<usize>,
) -> Result<Vec<Shard>, ShardError> {
    // Validate the `sizes` vector
    if sizes.iter().sum() != bytes.len() {
        return Err(ShardError::InvalidSplitSizes(GeneralError::new(
            format!(
                "{:?} is not a valid vector of byte split sizes",
                sizes,
            )
            .as_str(),
        )));
    }

    let mut shards: Vec<Shard>;
    let byte_pointer = 0usize;

    // Iterate through each size and create a shard with that data
    for size in sizes.iter() {
        let mut temp_bytes: Vec<u8> = Vec::new();

        for i in 0..size {
            temp_bytes.push(bytes[byte_pointer]);
            byte_pointer++;
        }
        shards.push(Shard::new(temp_bytes)?);
    }
}

impl PartialEq for Shard {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.size == other.size
            && self.timestamp == other.timestamp
            && self.id == other.id
    }
}
/*
impl Compressable for Shard { fn compress(&self) -> Vec<u8> {}
    fn decompress(bytes: Vec<u8>) -> Self {}
}
*/

impl crate::CanSerialize for Shard {
    type S = Self;
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    fn from_bytes(bytes: Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(&bytes[..])
    }
}

impl IsValue for Shard {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CanSerialize;

    #[test]
    fn test_to_bytes() {
        let shard = Shard::new(vec![1u8, 10u8]).unwrap();
        assert_eq!(shard.size, 2);

        println!("shard: {:?}", shard);
        shard.to_bytes().unwrap();
    }

    #[test]
    fn test_from_bytes() {
        let serialized =
            Shard::new(vec![1u8, 10u8]).unwrap().to_bytes().unwrap();
        /*
                let extra_bytes: &[u8] = &[
                    2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8, 2u8, 5u8,
                    2u8, 5u8, 2u8, 5u8,
                ];
                let serialized = [extra_bytes, &serialized[..]].concat();
        */
        let deserialized = Shard::from_bytes(serialized).unwrap();
        println!("deserialized shard: {:?}", deserialized);
    }
}
