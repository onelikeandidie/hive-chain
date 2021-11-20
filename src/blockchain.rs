use std::collections::HashSet;

use crate::{Block, Hashable, block, Hash};

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    blocks: Vec<Block>,
    next_difficulty: u128,
    next_fee: u128,
    unspent_outputs: HashSet<Hash>
}

impl Blockchain {
    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn add(&mut self, new_block: Block) -> Result<(), BlockValidationErr> {
        match self.verify_block_addition(&new_block) {
            Ok(new_unspent_outputs) => {
                self.unspent_outputs = new_unspent_outputs
            },
            Err(error) => {
                return Err(error);
            }
        }

        match self.get_last_block() {
            Some(_last_block) => {
                //let time_per_block = new_block.timestamp - last_block.timestamp;
                //let desired_blocks_per_second = 500.0;
                ////let desired_difficulty = desired_blocks_per_second - time_per_block;
                //let millisecs_per_sec = 1000;
                //let blocks_per_second: f32 = time_per_block as f32 * 1000.0;
                //println!("{}", blocks_per_second);
                ////if time_per_block < desired_time_per_block {
                ////    self.next_difficulty = self.next_difficulty * 2;
                ////} else if time_per_block > 500 {
                ////    self.next_difficulty = self.next_difficulty / 2;
                ////}
                //self.next_difficulty = self.next_difficulty >> blocks_per_second as u128;
            },
            None => {
                // This happens on the genesis block
                self.next_difficulty = !new_block.difficulty.clone().swap_bytes();
                println!("Genesis Block set difficulty to {}", self.next_difficulty);
            },
        }

        self.blocks.push(new_block.clone());

        Ok(())
    }

    pub fn get_difficulty(&self) -> u128 {
        (u128::MAX - self.next_difficulty).swap_bytes()
    }

    pub fn get_fee(&self) -> u128 {
        self.next_fee
    }

    pub fn verify_block_addition(&self, new_block: &Block) -> Result<HashSet<Hash>, BlockValidationErr> {
        // STEPS:
        // 1. Check index
        // 2. Check if hash matches
        // 3. Check if time passed
        // 4. Check genesis block format
        // 5. Check transactions
        // That's it
        let supposed_index: u32 = self.blocks.len() as u32;
        let hash = new_block.hash();
        if new_block.index != supposed_index { // Index check
            return Err(BlockValidationErr::MismatchedIndex)
        } else if !block::check_difficulty(&hash, new_block.difficulty) { // Hash matches
            return Err(BlockValidationErr::InvalidHash)
        } else if supposed_index != 0 {
            // This blocks timestamp
            let nb_t = new_block.timestamp;
            // Last blocks timestamp
            let lb_t = self.blocks[(supposed_index - 1) as usize].timestamp;
            let time_between_blocks = (nb_t - lb_t) as i128;
            if time_between_blocks <= 0 { // Time passes :c
                return Err(BlockValidationErr::AchronologicalTimestamp)
            }
        } else if new_block.prev_block_hash != vec![0;32] { // Check previous block matches
            return Err(BlockValidationErr::InvalidGenesisBlockFormat)
        }
        // Check transactions
        if let Some((coinbase, transactions)) = new_block.transactions.split_first() {
            // This checks if there is at least one transaction I think
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                let output_hashes = transaction.output_hashes();
                // We need to make sure that all of these 
                // are in the unspent hashes set
                // This compares the stuff in the transaction and
                // the unspent ones
                if !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput)
                }
                // Check if money wasn't printed
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }
                // Calculate the fee for the miner
                let fee = input_value - output_value;
                total_fee += fee;
                // Keep track of spent and traded coins
                block_spent.extend(input_hashes);
                block_created.extend(output_hashes);
            }
            // Check if there is enough space for the fee
            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            let mut new_unspent_outputs = self.unspent_outputs.clone();
            new_unspent_outputs.retain(|output| !block_spent.contains(output));
            new_unspent_outputs.extend(block_created);
            // Everything passed, return Ok
            return Ok(new_unspent_outputs)
        }
        Err(BlockValidationErr::InvalidCoinbaseTransaction)
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self { 
            blocks: Vec::new(), 
            next_difficulty: 0,
            next_fee: 1, 
            unspent_outputs: HashSet::new() 
        }
    }
}