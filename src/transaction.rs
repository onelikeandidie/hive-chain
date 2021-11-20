use std::collections::HashSet;
use std::fmt::Debug;

use crate::{Address, Hash, Hashable, util::u64_bytes};

#[derive(Clone)]
pub struct Output {
    pub address: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(self.address.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

impl Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.address, self.value)
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>
}

impl Transaction {
    pub fn input_value (&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    pub fn output_value (&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    pub fn input_hashes (&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect()
    }

    pub fn output_hashes (&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect()
    }

    pub fn is_coinbase (&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(&self.inputs
            .iter()
            .flat_map(|input| input.bytes())
            .collect::<Vec<u8>>());
        bytes.extend(&self.outputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>());

        bytes
    }
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction[{}:{}]:{:?} -> {:?}",
            self.input_value(),
            self.output_value(),
            self.inputs,
            self.outputs
        )
    }
}