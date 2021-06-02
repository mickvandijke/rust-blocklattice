use std::collections::{HashMap, VecDeque};
use crate::account::Account;
use crate::block::Block;
use crate::{GENESIS_BLOCK_HASH, BlockType, Height, Timestamp, GENESIS_ADDRESS};
use crate::now;

#[derive(Clone, Debug)]
pub struct Ledger {
    pub block_pool: HashMap<String, Block>,
    pub block_pool_queue: VecDeque<String>,
    pub cemented_blocks: HashMap<String, Block>,
    pub accounts: HashMap<String, Account>
}

impl Ledger {
    pub fn new () -> Self {
        Ledger {
            block_pool: HashMap::new(),
            block_pool_queue: VecDeque::new(),
            cemented_blocks: HashMap::new(),
            accounts: HashMap::new()
        }
    }

    pub fn verify_block_in_pool (&self, hash: &str) -> bool {
        self.block_pool.contains_key(hash)
    }

    pub fn verify_block_cemented (&self, hash: &str) -> bool {
        self.cemented_blocks.contains_key(hash)
    }

    pub fn verify_account_exists (&self, account: &str) -> bool {
        self.accounts.contains_key(account)
    }

    fn create_account_if_new (&mut self, account: &str) {
        self.accounts.entry(account.to_string()).or_insert(Account::new(account));
    }

    pub fn get_mut_account_from_address (&mut self, account: &str) -> Option<&mut Account> {
        if self.verify_account_exists(account) {
            Some(self.accounts.get_mut(account).unwrap())
        } else {
            None
        }
    }

    fn get_mut_block_from_hash (&mut self, hash: &str) -> Option<&mut Block> {
        if self.verify_block_cemented(hash) {
            Some(self.cemented_blocks.get_mut(hash).unwrap())
        } else if self.verify_block_in_pool(hash) {
            Some(self.block_pool.get_mut(hash).unwrap())
        } else {
            None
        }
    }

    fn cement_block (&mut self, block: Block) {
        if !self.verify_block_cemented(&block.calculate_hash()) {
            self.cemented_blocks.insert(block.calculate_hash().to_string(), block.clone());
            println!("Cemented block: {}, {:?}, in {} ms", block.block_type, block.calculate_hash(), (now() - block.timestamp));
        }
    }

    pub fn verify_block (&mut self, block: &Block) -> bool {
        let wrapped_account = self.get_mut_account_from_address(&block.account);
        if wrapped_account.is_none() { return false; }
        let account = wrapped_account.unwrap();

        if !account.verify_block_ownership(block) {
            return false;
        }

        let verify_block_amount: bool = match block.block_type {
            BlockType::Send => { self.verify_send_block(block) }
            BlockType::Receive => { self.verify_receive_block(block) }
        };

         verify_block_amount
    }

    fn verify_send_block (&mut self, block: &Block) -> bool {
        let wrapped_account = self.get_mut_account_from_address(&block.account);
        if wrapped_account.is_none() { return false; }
        let account = wrapped_account.unwrap();

        block.account != block.target && block.amount <= account.balance
    }

    fn verify_receive_block (&mut self, block: &Block) -> bool {
        let wrapped_target_block = self.get_mut_block_from_hash(&block.target);
        if wrapped_target_block.is_none() {
            println!("Target block: {} not found.", block.target);
            return false;
        }
        let target_block = wrapped_target_block.unwrap().clone();

        if !self.verify_block_cemented(&block.target) {
            println!("Target block: {} not confirmed.", block.target);
        }

        let wrapped_account = self.get_mut_account_from_address(&block.account);
        if wrapped_account.is_none() { return false; }
        let account = wrapped_account.unwrap();

        // Receive block verification checks
        if account.verify_block_ownership(&target_block) {
            println!("Can't receive own send block.");
            return false;
        } else if target_block.target.ne(&account.address) {
            println!("Target address ({}) does not match with account address ({}).", &target_block.target, &account.address);
            return false;
        } else if target_block.amount.ne(&block.amount) {
            println!("Receive block amount does not match the send block amount.");
            return false;
        } else if !account.pending_blocks.contains(&block.target) {
            println!("Send block has already been received.");
            return false;
        } else {
            return true;
        }
    }

    pub fn new_block (block_type: BlockType, height: Height, block_account: &str, previous_hash: &str, timestamp: Timestamp, target_account_or_block: &str, amount: u64) -> Block {
        match block_type {
            BlockType::Send => {
                Block::new_send(
                    height,
                    block_account,
                    previous_hash,
                    timestamp,
                    target_account_or_block,
                    amount)
            }
            BlockType::Receive => {
                Block::new_receive(
                    height,
                    block_account,
                    previous_hash,
                    timestamp,
                    target_account_or_block,
                    amount)
            }
        }
    }

    fn add_pending_block_to_account (&mut self, account: &str, block: &Block) {
        self.create_account_if_new(account);

        let wrapped_account = self.get_mut_account_from_address(account);

        if wrapped_account.is_some() {
            let account = wrapped_account.unwrap();
            account.add_pending_block_hash(&block.calculate_hash());

        } else {
            println!("Something went wrong when getting account.");
        }
    }

    pub fn try_block (&mut self, block: Block) {
        if !self.verify_block_cemented(&block.calculate_hash()) {
            self.create_account_if_new(&block.account);

            if self.verify_block(&block) {
                self.cement_block(block.clone());

                let wrapped_account = self.get_mut_account_from_address(&block.account);

                if wrapped_account.is_some() {
                    let account = wrapped_account.unwrap();
                    account.add_block(block.clone());
                    match &block.block_type {
                        BlockType::Send => { self.add_pending_block_to_account(&block.target, &block); }
                        BlockType::Receive => {}
                    };
                } else {
                    println!("Something went wrong when getting account.");
                }
            } else {
                println!("Block: {}, rejected.", block.calculate_hash());
            }
        } else {
            println!("Block: {}, already cemented.", block.calculate_hash());
        }
    }

    // target_account_or_block expects a block hash with a receive block and an account with a send block
    pub fn try_new_block (&mut self, block_type: BlockType, block_account: &str, target_account_or_block: &str, amount: u64) {
        self.create_account_if_new(block_account);

        let wrapped_account = self.get_mut_account_from_address(block_account);

        if wrapped_account.is_some() {
            let account = wrapped_account.unwrap();

            let block: Block = Ledger::new_block(block_type, &account.block_height() + 1, &account.address, &account.last_block_hash(), now(), target_account_or_block, amount);

            self.try_block(block);
        } else {
            println!("Something went wrong when getting account.");
        }
    }

    pub fn try_new_send_block (&mut self, block_account: &str, target_account: &str, amount: u64) {
        self.try_new_block(BlockType::Send, block_account, target_account, amount);
    }

    pub fn try_new_receive_block (&mut self, block_account: &str, target_block: &str, amount: u64) {
        self.try_new_block(BlockType::Receive, block_account, target_block, amount);
    }

    pub fn new_account(&mut self, account: &str) {
        let new_account = Account::new(&account);

        self.accounts.insert(
            (&new_account.address).to_string(),
            new_account
        );
        assert_eq!(self.verify_account_exists(&account), true);
    }

    pub fn create_genesis_account (&mut self) {
        let mut genesis_account = Account::new(GENESIS_ADDRESS);

        let genesis_block = Block::new_genesis();

        &genesis_account.add_block(genesis_block.clone());
        assert_eq!(genesis_account.last_block_hash(), GENESIS_BLOCK_HASH);

        self.accounts.insert(
            genesis_account.address.to_string(),
            genesis_account
        );
        assert_eq!(self.verify_account_exists(GENESIS_ADDRESS), true);

        self.cement_block(genesis_block.clone());
    }
}
