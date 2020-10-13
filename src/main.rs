mod bitvec;
mod block;
mod blocks;

use blocks::Blocks;

fn main() {
    // initial loading data in Hamming code and assign parity
    let mut blocks : Blocks = Blocks::from("a bit longer string");
    println!("Before noise:\t {}", &blocks.to_string());

    // introduce noise randomly to the blocks and see its output
    blocks.introduce_noise();
    println!("After noise:\t {}", &blocks.to_string());

    // then repair using the parity
    blocks.repair();
    println!("Repaired:\t {}", &blocks.to_string());
}

