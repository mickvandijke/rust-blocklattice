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
        &self.cemented_blocks.insert(block.calculate_hash().to_string(), block.clone());
        println!("Cemented block: {:?}, in {} ms", block.calculate_hash(), (now() - block.timestamp));
    }

    pub fn verify_block (&self, block: &Block) -> bool {
        let verify_block_amount: bool = match block.block_type {
            BlockType::Send => { self.verify_send_block(block) }
            _ => { true }
        };

        self.verify_block_headers() && verify_block_amount
    }

    fn try_block (&mut self, block_type: BlockType, block_account: &str, target_account: &str, amount: u64) {
        &self.accounts.entry(block_account.to_string()).or_insert(Account::new(block_account));

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

        if account.verify_block(&block) {
            &account.add_block(block.clone());
            &self.cement_block(block.clone());
        }
    }

    pub fn new_send_block (&mut self, block_account: &str, target_account: &str, amount: u64) {
        &self.try_block(BlockType::Send, block_account, target_account, amount);
    }

    pub fn verify_account_exists(&self, account: &str) -> bool {
        self.accounts.contains_key(account)
    }

    pub fn new_account(&mut self, account: &str) {
        let new_account = Account::new(&account);

        &self.accounts.insert(
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

        &self.cement_block(genesis_block.clone());
    }
}
