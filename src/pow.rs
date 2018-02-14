extern crate num;
extern crate ring;

use self::num::BigUint;
use self::ring::digest::{ digest, SHA256 };
use std::cmp::Ordering;
use std::fmt::Write;
use block::Block;

const targetBits = 24

pub struct ProofOfWork<'blk> {
    block: &'blk Block,
    target: BigUint,
}

impl<'a> ProofOfWork<'a> {
    pub fn new<'blk>(blk: &'blk Block) -> ProofOfWork {
        ProofOfWork {
            block: blk,
            target: {
                let mut big_int_vec = vec![];
                for _ in 0..256 {
                    big_int_vec.push(0);
                }
                BigUint::new(big_int_vec)
            }
        }
    }

    fn prepare_data(&self, nonce: u64) -> String {
        let mut data: String = "".to_string();
        write!(&mut data, "{:?}{}{}{}{}", 
               self.block.prev_block_hash.as_slice(),
               self.block.data,
               self.block.time_stamp,
               self.target,
               nonce).unwrap();
        data
    }
    pub fn run(&self) -> (u64, Vec<u8>) {
        let mut hash_int = BigUint::new(vec![]);
        let mut hash: Vec<u8> = vec![];
        let mut nonce = 0;

        println!("Mining Block containing: {}\n", self.block.data);

        while nonce < u64::max_value() {
            let data = self.prepare_data(nonce);
            let hash = digest(&SHA256, data.as_ref());

            hash_int = match BigUint::parse_bytes(hash.as_ref(), 10) {
                Some(n) => n,
                None => hash_int,
            };
            
            match hash_int.cmp(&self.target) {
                Ordering::Less => break,
                Ordering::Greater => nonce += 1,
                Ordering::Equal => {}
            }
        }

        drop(self.block);

        (nonce, hash.to_vec())
    }
}
