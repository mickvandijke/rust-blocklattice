pub mod block;
pub mod account;
pub mod ledger;

use std::time::{ SystemTime, UNIX_EPOCH };
use std::fmt::{ self, Debug };

pub const GENESIS_AMOUNT: u64 = 117000000;
pub const GENESIS_BLOCK_HASH: &str = "debad8e36c3464e3ec15380d7ab709e46a4ae4d701d19756f3416ddf398f6007";

pub type Height = u64;
pub type Address = String;

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        ;

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
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
