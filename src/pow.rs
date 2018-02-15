extern crate num;
extern crate ring;

use self::num::BigUint;
use self::ring::digest::{ digest, SHA256 };
use std::cmp::Ordering;
use std::fmt::Write;
use block::Block;

const TARGET_BITS: u64 = 3;

pub fn run_pow(blk: &mut Block) {
    
    let mut hash_int;
    let mut hash: Vec<u8> = vec![];
    let mut nonce = 0u64;
    
    let target = {
        let mut big_int_vec = vec![];
        for i in 0..TARGET_BITS - 1 {
            big_int_vec.push(0);
        }
        big_int_vec.push(1);
        BigUint::new(big_int_vec)
    };

    println!("Mining Block containing: {}\n", blk.data);

    while nonce < u64::max_value() {
        let data = {
            let mut data: String = "".to_string();
            write!(&mut data, "{:?}{}{}{}{}", 
                   blk.prev_block_hash.as_slice(),
                   blk.data,
                   blk.time_stamp,
                   TARGET_BITS,
                   nonce).unwrap();
            data
        };

        hash = digest(&SHA256, data.as_ref()).as_ref().to_vec();
        hash_int = BigUint::from_bytes_le(hash.as_ref());

        println!("Nonce: {}\n", nonce);
        println!("HI: {:x} TARGET: {:x}\n\n", hash.as_slice(), target);
        
        match hash_int.cmp(&target) {
            Ordering::Less => break,
            Ordering::Greater => nonce += 1,
            Ordering::Equal => {}
        }
    }

    println!("Nonce: {}", nonce);

    blk.hash  = hash;
    blk.nonce = nonce;
}
