use rust_blocklattice::ledger::Ledger;

fn main() {
    let mut ledger = Ledger::new();
    &ledger.create_genesis_account();
    &ledger.new_send_block("genesis", "genesis2", 32);
    &ledger.new_receive_block("genesis2", "genesis", 16);
    print!("{:?}", &ledger);
}
