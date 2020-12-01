#![allow(dead_code)]
use rand;

use std::iter::FromIterator;

use super::bitvec::BitVec;
use super::block::Block;

pub struct Blocks(Vec<Block>);
impl From<&str> for Blocks {
    fn from(data: &str) -> Blocks {
        Blocks::new(data.as_bytes(), false)
    }
}

impl Blocks {
    /// `new` will take bytes and convert them to 16 bit blocks where 11 bits
    /// are used to represent data and 5 bits are reseved for parity bits.
    ///
    /// pass is_raw to indicate the input data already contains parity bits.
    /// the `new` function will then skipp leaving space for them
    pub fn new(data: &[u8], is_raw: bool) -> Self {
        // allocate block buffer
        let mut blocks = Vec::new();

        // make the data iterable per byte
        let bits = BitVec::from_bytes(data);

        let mut bits = bits.peekable();

        // iterate until theres no more bits
        while bits.peek().is_some() {
            let block = (0..15usize)
                .rev()
                .filter(|bit| is_raw || !usize::is_power_of_two(15 - bit))
                .fold(0u16, |block, bit| match bits.next() {
                    Some(true) => block | 0b1 << bit,
                    Some(false) => block | 0b0 << bit,
                    None => block,
                });

            blocks.push(Block::from(block));
        }

        Blocks(blocks)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.0.iter()
    }

    pub fn introduce_noise(&mut self) {
        for block in &mut self.0 {
            if rand::random() {
                block.flip_random_bit();
            }
        }
    }

    pub fn repair(&mut self) {
        for block in &mut self.0 {
            block.repair();
        }
    }

    /// return the blocks in a vec of bools. Optionally strip out
    /// the parity bits by passing `strip_from_parity` true.
    fn serialize(&self, strip_from_parity: bool) -> Vec<bool> {
        self.iter()
            .flat_map(|block| {
                (1..16usize)
                    .filter(|bit| !(strip_from_parity && bit.is_power_of_two()))
                    .map(move |bit| block.data >> (15 - bit) & 0b1 > 0)
            })
            .collect()
    }

    /// return block as vec of u8 (bytes)
    pub fn to_byte_vec(&self) -> Vec<u8> {
        self.serialize(false)
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0u8, |byte, (idx, bit)| match bit {
                        true => byte | 0b1 << 7 - idx,
                        false => byte | 0b0 << 7 - idx,
                    })
            })
            .collect::<Vec<u8>>()
    }

    /// convert data back to readable string stripping each block from its parity
    pub fn to_string(&self) -> String {
        // chunk per 8 bits to turn into bytes
        let chars: Vec<char> = self
            .serialize(true)
            .chunks(8)
            .filter(|chunk| chunk.len() >= 8)
            .filter_map(|chunk| {
                let byte = chunk
                    .iter()
                    .enumerate()
                    .fold(0u8, |byte, (idx, bit)| match bit {
                        true => byte | 0b1 << 7 - idx,
                        false => byte | 0b0 << 7 - idx,
                    });

                match byte {
                    byte if byte > 0 => Some(char::from(byte)),
                    _ => None,
                }
            })
            .collect();

        String::from_iter(chars)
    }
}

#[cfg(test)]
mod blocks_test {
    use super::Blocks;
    #[test]
    fn new_passed_bytes_returns_blocks_leaving_space_for_parity() {
        let data = "some bytes";
        let blocks = Blocks::new(data.as_bytes(), false);

        // eleven data bits per block should yield 8 blocks for 10 bytes
        let expected_num_blocks = (data.len() as f64 * 8.0 / 11.0).ceil() as usize;

        // expect 8 blocks
        assert_eq!(blocks.iter().as_slice().len(), expected_num_blocks);
    }

    #[test]
    fn new_passed_bytes_and_raw_leaves_bits_untouched() {
        let data = "some bytes".as_bytes();
        let blocks = Blocks::new(&data, true);

        // ten bytes original, should return 5 blocks
        assert_eq!(blocks.iter().as_slice().len(), 5);
    }

    #[test]
    fn into_string_returns_stringified_version_of_original_input() {
        let data = "blah";
        let blocks = Blocks::new(data.as_bytes(), false);
        assert_eq!(blocks.to_string(), data);
    }
}
