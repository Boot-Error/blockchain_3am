extern crate clap;

mod block;
mod pow;

use clap::{Arg, App};
use block::BlockChain;

fn main() {
    let matches = App::new("Blkchain_3am")
            .version("1.0.0")
            .author("Boot-Error <booterror99@gmail.com>")
            .about("Tiny blockchain implementation")
            .arg(Arg::with_name("add")
                    .short("a")
                    .long("add")
                    .takes_value(true)
                    .help("Add new block, input some text"))
            .get_matches();

    let new_data = matches.value_of("add").unwrap();

    // blockchain
    let mut bc = BlockChain::new();

    bc.add_block(new_data.to_string());
    bc.show_blocks();
}
