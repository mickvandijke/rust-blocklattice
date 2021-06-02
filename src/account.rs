use crate::block::Block;
use crate::{Address, Height, BlockType};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Account {
    pub address: Address,
    pub balance: u64,
    pub blockchain: Vec<Block>,
    pub pending_blocks: HashSet<String>
}

impl Account {
    pub(crate) fn new (address: &str) -> Self {
        Account {
            address: address.to_string(),
            balance: 0,
            blockchain: Vec::new(),
            pending_blocks: HashSet::new()
        }
    }

    pub fn add_block (&mut self, block: Block) {
        self.blockchain.push(block.clone());
        match block.block_type {
            BlockType::Send => { self.balance -= block.amount }
            BlockType::Receive => {
                self.remove_pending_block_hash(&block.target);
                assert_eq!(self.pending_blocks.contains(&block.target), false);

                self.balance += block.amount
            }
        }
    }

    pub fn add_pending_block_hash (&mut self, hash: &str) {
        self.pending_blocks.insert(hash.to_string());
    }

    pub fn remove_pending_block_hash (&mut self, hash: &str) {
        self.pending_blocks.remove(hash);
    }

    pub fn verify_block_ownership (&self, block: &Block) -> bool {
        block.height == (self.blockchain.len() + 1) as Height
        && block.account == self.address
        && block.previous_hash == self.last_block_hash()
        // TODO: Add signature check.
    }

    pub fn block_height (&self) -> Height {
        self.blockchain.len() as Height
    }

    pub fn last_block_hash (&self) -> String {
        let mut hash: String = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        if self.blockchain.last().is_some() {
            hash = self.blockchain.last().unwrap().calculate_hash();
        }

        return hash;
    }
}
