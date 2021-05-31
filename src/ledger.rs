use std::collections::{HashMap, VecDeque};
use crate::account::Account;
use crate::block::Block;
use crate::{GENESIS_BLOCK_HASH, BlockType, Height};
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

    fn get_account_from_address(&mut self, account: &str) -> &mut Account{
        let mutable_account_ref = self.accounts.get_mut(account).unwrap();
        assert_eq!(mutable_account_ref.address, account);
        return mutable_account_ref;
    }

    fn cement_block (&mut self, block: Block) {
        self.cemented_blocks.insert(block.calculate_hash().to_string(), block.clone());
        println!("Cemented block: {:?}, in {} ms", block.calculate_hash(), (now() - block.timestamp));
    }

    pub fn verify_block (&mut self, block: &Block) -> bool {
        let account = self.get_account_from_address(&block.account);

        if !account.verify_block_ownership(block) {
            return false
        }

        let verify_block_amount: bool = match block.block_type {
            BlockType::Send => { self.verify_send_block(block) }
            _ => { true }
        };

         verify_block_amount
    }

    fn verify_send_block (&mut self, block: &Block) -> bool {
        let account = self.get_account_from_address(&block.account);

        block.account != block.target_account && block.amount <= account.balance
    }

    fn try_block (&mut self, block: Block) {
        if self.verify_block(&block) {
            self.cement_block(block.clone());
            let account = self.get_account_from_address(&block.account);
            account.add_block(block.clone());
        }
         else {
             println!("Block: {}, rejected.", block.calculate_hash());
         }
    }

    fn new_block (&mut self, block_type: BlockType, block_account: &str, target_account: &str, amount: u64) -> Block {
        self.accounts.entry(block_account.to_string()).or_insert(Account::new(block_account));

        let account = self.get_account_from_address(block_account);

        let block: Block = match block_type {
            BlockType::Send => {
                Block::new_send(
                    (&account.blockchain.len() + 1) as Height,
                    &account.address,
                    &account.last_block_hash(),
                    now(),
                    target_account,
                    amount)
            }
            BlockType::Receive => {
                Block::new_receive(
                    (&account.blockchain.len() + 1) as Height,
                    &account.address,
                    &account.last_block_hash(),
                    now(),
                    target_account,
                    amount)
            }
        };

        return block;
    }

    pub fn new_send_block (&mut self, block_account: &str, target_account: &str, amount: u64) {
        let block = self.new_block(BlockType::Send, block_account, target_account, amount);
        self.try_block(block);
    }

    pub fn new_receive_block (&mut self, block_account: &str, target_account: &str, amount: u64) {
        let block = self.new_block(BlockType::Receive, block_account, target_account, amount);
        self.try_block(block);
    }

    pub fn verify_account_exists(&self, account: &str) -> bool {
        self.accounts.contains_key(account)
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
        let mut genesis_account = Account::new("genesis");

        let genesis_block = Block::new_genesis();

        &genesis_account.add_block(genesis_block.clone());
        assert_eq!(genesis_account.last_block_hash(), GENESIS_BLOCK_HASH);

        self.accounts.insert(
            genesis_account.address.to_string(),
            genesis_account
        );
        assert_eq!(self.verify_account_exists("genesis"), true);

        self.cement_block(genesis_block.clone());
    }
}
