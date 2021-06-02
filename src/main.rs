use rust_blocklattice::ledger::Ledger;
use rust_blocklattice::{GENESIS_ADDRESS};

fn main() {
    let mut ledger = Ledger::new();
    &ledger.create_genesis_account();
    &ledger.try_new_send_block(GENESIS_ADDRESS, "genesis2", 32);
    let send_block_hash = &ledger.get_mut_account_from_address(GENESIS_ADDRESS).unwrap().last_block_hash().to_owned();
    &ledger.try_new_receive_block("genesis2", send_block_hash, 32);
    &ledger.try_new_receive_block("genesis2", send_block_hash, 32);
    print!("{:?}", &ledger);
}
