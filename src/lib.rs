pub mod block;
pub mod account;
pub mod ledger;

use std::time::{ SystemTime, UNIX_EPOCH };
use std::fmt::{ self, Debug };

pub const GENESIS_ADDRESS: &str = "genesis";
pub const GENESIS_AMOUNT: u64 = 117000000;
pub const GENESIS_BLOCK_HASH: &str = "debad8e36c3464e3ec15380d7ab709e46a4ae4d701d19756f3416ddf398f6007";

pub type Height = u64;
pub type Address = String;
pub type Timestamp = u64;

pub fn now () -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

#[derive(Clone, Debug)]
pub enum BlockType {
    Send,
    Receive,
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
