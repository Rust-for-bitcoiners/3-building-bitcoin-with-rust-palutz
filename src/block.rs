#![allow(unused)]
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use std::collections::LinkedList as List;

static BLOCK_MAGIC: u32 = 0xd9b4bef9;

struct BlockChain {
    blocks: List<Block>
}

impl BlockChain {
    fn block_height(self) -> usize {
        self.blocks.len()
    }
}


#[derive(Debug, Serialize)]
pub struct Amount(u64);

#[derive(Debug)]
struct Block {
    id: u128,
    magic: u32,
    block_size: u32,
    block_header: BlockHeader,
    transactions: List<Transaction>,
    hash: String,
}

impl Block {
    pub fn transaction_count(self) -> u32 {
        self.transactions.len() as u32
    }

    pub fn version(self) -> u32 {
        self.block_header.version
    }
}

#[derive(Debug, Serialize)]
// need to implemnet encode and decode for block header and block
struct BlockHeader {
    version: u32,
    hashprev_block: u128,
    hash_merkle_root: u128,
    time: u32,
    bits: u32,
    nonce: u32,
}

#[derive(Debug)]
struct Transaction {
    version: u32,
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    lock_time: u32,
    txid: String,
}

impl Transaction {
    pub fn txid(&self) -> String {
        let mut txid_data = Vec::new();
        // encode version and lock time...
        txid_data.push(self.version.to_string());
        txid_data.push(self.lock_time.to_string());
        txid_data.connect("")
    }
}


#[derive(Debug, Serialize)]
struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

#[derive(Debug, Serialize)]
struct TxOut {
    public_address: String,
    satoshis: u64,
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
