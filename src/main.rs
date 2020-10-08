// TODO:
// - Refactor. Better iterators
// - Blogpost
//
mod bitvec;

use rand;
use bitvec::BitVec;

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

struct Block {
    data: u16
}

impl Block {
    fn from(data: u16) -> Self {
        let mut block = Block {
            data: data
        };

        block.assign_parity(&block.parity());

        block
    }

    fn parity(&self) -> u8 {
        (0..16)
            .filter(|bit| &self.data >> 15 - bit & 0b1 > 0)
            .fold(0u8, |acc, bit| acc ^ bit)
    }

    fn assign_parity(&mut self, parity: &u8){
        self.data =
            (0..4).rev()
                .fold(self.data, |acc, bit| {
                    let parity_bit = match parity >> bit & 0b1 > 0 {
                        true => 0b1,
                        false => 0b0
                    };

                    acc | parity_bit << 15 - 2u8.pow(bit)
                });
    }

    fn repair(&mut self) {
        match self.parity() {
            0 => (),
            err => {
                self.data ^= 0b1 << 15 - err as u16;
            },
        }
    }

    fn flip_random_bit(&mut self) {
        self.data ^= 0b1 << rand::random::<u8>() % 15
    }
}

struct Blocks(Vec<Block>);
impl From<&str> for Blocks {
    fn from(data: &str) -> Blocks {
        Blocks::new(data.as_bytes())
    }
}

impl Blocks {
    fn new(data: &[u8]) -> Self {
        // allocate block buffer
        let mut blocks = Vec::new();

        // make the data ierable per byte
        let mut bits = BitVec::from_bytes(data).peekable();

        // iterate until theres no more bits
        while bits.peek().is_some() {
            let block = (0..15).rev()
                .filter(|bit|  !usize::is_power_of_two(15 - *bit))
                .fold(0u16, |block, bit| {
                    match bits.next() {
                        Some(true) =>  block | 0b1 << bit,
                        Some(false) => block | 0b0 << bit,
                        None => block
                    }
                });

            blocks.push(Block::from(block))
        }

        Blocks(blocks)
    }

    fn introduce_noise(&mut self) {
        for block in &mut self.0 {
            if rand::random() {
                block.flip_random_bit();
            }
        }
    }

    fn repair(&mut self) {
        for block in &mut self.0 {
            block.repair();
        }
    }

    fn to_string(&self) -> String {
        // TODO: Refactor this to a more functional style
        let mut bits:Vec<bool> = Vec::new();

        // convert the blocks into a long stream of booleans
        for block in &self.0 {
            for bit in 1..16 {

                // don't read the parity bits as data
                if usize::is_power_of_two(bit) {
                    continue;
                }

                bits.push(block.data >> 15 - bit & 0b1 > 0)
            }
        }

        let mut bytes: Vec<char> = Vec::new();

        // then from that vector; chunks of 8 to get the bits
        for chunk in bits.chunks(8) {
            if chunk.len() < 8 {
                continue;
            }
            let mut byte = 0u8;

            for (idx, bit) in chunk.iter().enumerate() {
                match bit {
                    true =>  byte |= 0b1 << 7 - idx,
                    false => byte |= 0b0 << 7 - idx,
                }
            }

            &bytes.push(char::from(byte));
        }


        bytes.into_iter().collect()
    }
}

// This should be the fmt::Display trait? Then you can just use it as println!()?
fn print_block(block: &u16) {
    for bit in 0..16 {

        // for every four bits, start on new line
        if bit % 4 == 0 {
            print!("\n")
        }

        // determine the value
        let value = match block >> 15 - bit & 0b1 > 0 {
            true => "1",
            false => "0"
        };

        // color the parity bits different
        if usize::is_power_of_two(bit) {
            print!("\x1b[1;96;127m{} \x1b[0m", value);
        } else {
            print!("{} ", value)
        }
    }

    print!("\n")
}

