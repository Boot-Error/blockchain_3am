extern crate base58;

use pow::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{ fmt, str };
use self::base58::ToBase58;

macro_rules! genesis_block {
    () => { 
        Block::new("Genesis Block".to_string(), vec![]);
    }
}

#[derive(Debug)]
pub struct Block {
   pub time_stamp: u64,
   pub data: String,
   pub nonce: u64,
   #[derive(ToBase58)]
   pub prev_block_hash: Vec<u8>,
   pub hash: Vec<u8>,
}

impl Block {
    pub fn new(data: String, prev_block_hash: Vec<u8>) -> Block {
        let mut b = Block {
            time_stamp: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0 
            },
            data: data,
            prev_block_hash: prev_block_hash,
            hash: vec![],
            nonce: 0u64,
        };

        let pow = ProofOfWork::new(&b);
        let hardwork = pow.run();
        b.nonce = hardwork.0;
        b.hash  = hardwork.1;
        // b.hash = digest(&SHA256, {
        //     let mut hashstr: String = "".to_string();
        //     write!(&mut hashstr, "{}{}{:?}", b.time_stamp, b.data, b.prev_block_hash).unwrap();
        //     hashstr
        // }.as_ref()).as_ref().to_vec();
        
        b
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
               "Block {:?}\n\tTimeStamp: {}\n\tData: {}\n\tPrevBlock: {:?}\n\n",
               self.hash.to_base58(), self.time_stamp, self.data, self.prev_block_hash.to_base58()
            )
    }
}


#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![genesis_block!()]
        }
    }
    pub fn add_block(&mut self, data: String) {
        let prev_block = match self.blocks.pop() {
            Some(x) => x,
            None => genesis_block!()
        };
        let new_block  = Block::new(data, prev_block.hash.clone());
        self.blocks.push(prev_block);
        self.blocks.push(new_block);
    }
    pub fn show_blocks(&self) {
        for blk in self.blocks.iter() {
            println!("{}", blk);
        }
    }
}
