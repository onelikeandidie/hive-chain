use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::Hash;
use crate::Transaction;
use crate::hashable::Hashable;
use crate::util::difficulty_bytes_as_u128;
use crate::util::u128_bytes;
use crate::util::u32_bytes;
use crate::util::u64_bytes;

#[derive(Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: Hash,
    pub hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {}(...) at: {} with: {} transaction nonce: {}",
            &self.index,
            &hex::encode(&self.hash).to_string()[..4],
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash,
        transactions: Vec<Transaction>, difficulty: u128) -> Self
    {
        Block { 
            index, 
            timestamp, 
            prev_block_hash, 
            hash: vec![0; 32],
            nonce: 0,
            transactions, 
            difficulty }
    }

    pub fn mine (&mut self) {
        for nonce_attempt in 0..u64::MAX {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions
            .iter()
            .flat_map(|tx| tx.bytes())
            .collect::<Vec<u8>>());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash.to_vec())
}