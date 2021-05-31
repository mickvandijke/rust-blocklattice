use crypto_hash::{Algorithm, hex_digest};
use crate::{BlockType, Height, Timestamp};
use super::GENESIS_AMOUNT;

#[derive(Clone, Debug)]
pub struct Block {
    pub height: Height,
    pub block_type: BlockType,
    pub previous_hash: String,
    pub timestamp: Timestamp,
    pub account: String,
    pub target_account: String,
    pub amount: u64,
}

impl Block {
    pub fn new (height: Height, account: &str, block_type: BlockType, previous_hash: &str, timestamp: Timestamp, target_account: &str, amount: u64) -> Self {
        Block {
            height,
            block_type,
            previous_hash: previous_hash.to_string(),
            timestamp,
            account: account.to_string(),
            target_account: target_account.to_string(),
            amount
        }
    }

    pub fn new_send (height: Height, account: &str, previous_hash: &str, timestamp: Timestamp, target_account: &str, amount: u64) -> Self {
        Block::new(height, account, BlockType::Send, previous_hash, timestamp, target_account, amount)
    }

    pub fn new_receive (height: Height, account: &str, previous_hash: &str, timestamp: Timestamp, target_account: &str, amount: u64) -> Self {
        Block::new(height, account, BlockType::Receive, previous_hash, timestamp, target_account, amount)
    }

    pub fn new_genesis () -> Self {
        Block::new_receive(1,"genesis", "", 0, "", GENESIS_AMOUNT)
    }

    pub fn calculate_hash (&self) -> String {
        let mut hashable_string: String = "".to_string();

        hashable_string.push_str(&self.height.to_string());
        hashable_string.push_str(&self.block_type.to_string());
        hashable_string.push_str(&self.previous_hash.to_string());
        hashable_string.push_str(&self.timestamp.to_string());
        hashable_string.push_str(&self.account.to_string());
        hashable_string.push_str(&self.target_account.to_string());
        hashable_string.push_str(&self.amount.to_string());

        hex_digest(Algorithm::SHA256, hashable_string.as_bytes())
    }
}


