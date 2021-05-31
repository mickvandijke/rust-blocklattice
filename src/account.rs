use crate::block::Block;
use crate::{Address, Height};

#[derive(Clone, Debug)]
pub struct Account {
    pub address: Address,
    pub blockchain: Vec<Block>,
    pub balance: u128
}

impl Account {
    pub(crate) fn new (address: &str) -> Self {
        Account {
            address: address.to_string(),
            blockchain: Vec::new(),
            balance: 0
        }
    }

    pub fn add_block (&mut self, block: Block) {
        &self.blockchain.push(block);
    }

    pub fn verify_block (&self, block: &Block) -> bool {
        block.height == (&self.blockchain.len() + 1) as Height
        && block.account == self.address
        && block.previous_hash == self.last_block_hash()
    }

    pub fn last_block_hash (&self) -> String {
        let mut hash: String = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        if self.blockchain.last().is_some() {
            hash = self.blockchain.last().unwrap().calculate_hash();
        }

        return hash;
    }
}
