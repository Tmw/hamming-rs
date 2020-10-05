// TODO: 
// refactor. Better iterators, do bit shifting the better way, etc..
//

use rand;

struct BitVec {
    source: Vec<u8>,
    index: usize
}

impl BitVec {
    fn from_bytes(data: &[u8]) -> Self {
        BitVec {
            source: Vec::from(data),
            index: 0
        }
    }
}

impl Iterator for BitVec {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let byte_index = self.index / 8;
        let bit_index = self.index % 8;

        if let Some(current_byte) = self.source.get(byte_index) {
            self.index = self.index + 1;
            Some(current_byte & (0b1 << 7 - bit_index) > 0)
        } else {
            None
        }
    }
}

fn main() {
    let blocks : Blocks = Blocks::from("This long and too long much more and now it works for some reason?");

    let blocks = Blocks(blocks.0.iter().map(|block| {
        let parity = calculate_parity(block);
        assign_parity(block, &parity)
    }).collect());


    let out = to_string(&blocks);
    println!("Decode without noise:\t {}", out);

    // introduce some noise
    let with_noise = Blocks(blocks.0.iter().map(flip_random_bit).collect());

    let disturbed_out = to_string(&with_noise);
    println!("decode with noise:\t {}", disturbed_out);

    let repaired = repair(&with_noise);
    println!("Repaired:\t\t {}", to_string(&repaired));
}

struct Blocks (Vec<u16>);
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
            let block = (1..16)
                .filter(|i|  !usize::is_power_of_two(*i))
                .fold(0u16, |acc, i| {
                    match bits.next() {
                        Some(true) =>  acc | (0b1 << 15 - i),
                        Some(false) => acc | (0b0 << 15 - i),
                        None => acc
                    }
                });

            blocks.push(block)
        }

        Blocks(blocks)
    }
}

fn repair(blocks: &Blocks) -> Blocks {
    Blocks(blocks.0.iter().map(|block| {

        match calculate_parity(&block) {
            0 => *block,
            err => block ^ (0b1 << 15 - err as u16),
        }



    }).collect())
}

fn flip_random_bit(block: &u16) -> u16 {
    match rand::random() {
        true => block.to_owned() ^ 0b1 << rand::random::<u8>() % 15,
        false => block.to_owned(),
    }
}

fn to_string(blocks: &Blocks) -> String {
    let mut bits:Vec<bool> = Vec::new();

    // convert the blocks into a long stream of booleans
    for block in &blocks.0 {
        for bit in 1..16 {

            // don't read the parity bits as data
            if usize::is_power_of_two(bit) {
                continue;
            }

            bits.push(block & 0b1 << 15 - bit > 0)
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
                true =>  byte |= 0b1 << 7- idx,
                false => byte |= 0b0 << 7 - idx,
            }
        }

        &bytes.push(char::from(byte));
    }


    bytes.into_iter().collect()
}


fn calculate_parity(data: &u16) -> u8 {
    (0..16)
        .filter(|bit| data & (0b1 << 15 - bit) as u16 > 0)
        .fold(0u8, |acc, bit| acc ^ bit)
}

// assign parity bits in using big endian
fn assign_parity(data: &u16, parity: &u8) -> u16 {
    (0..4)
        .fold(*data, |acc, bit| {
            let parity_bit = match parity & (0b1 << bit) > 0 {
                true => 0b1,
                false => 0b0
            };

            acc | parity_bit << 15 - 2u8.pow(bit)
        })
}

fn print_block(block: &u16) {
    for bit in 0..16 {

        // for every four bits, start on new line
        if bit % 4 == 0 {
            print!("\n")
        }

        // determine the value
        let value = match block & (0b1 << 15 - bit) > 0 {
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

