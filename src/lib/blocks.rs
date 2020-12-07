#![allow(dead_code)]
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
    /// Pass is_raw to indicate the input data already contains parity bits,
    /// the 16 bits will then be assigned per block as is.
    pub fn new(data: &[u8], is_raw: bool) -> Self {
        let mut blocks = Vec::new();
        let mut bits = BitVec::from_bytes(data).peekable();

        // iterate until theres no more bits
        while bits.peek().is_some() {
            let block = (0..16u8)
                .rev()
                .filter(|bit| is_raw || !Blocks::is_parity_bit(15 - bit))
                .fold(0u16, |block, bit| match bits.next() {
                    Some(true) => block | 0b1 << bit,
                    Some(false) => block | 0b0 << bit,
                    None => block,
                });

            blocks.push(Block::from(block));
        }

        Blocks(blocks)
    }

    // reserve 0th bit for parity; even though it wont be utilized for now.
    fn is_parity_bit(index: u8) -> bool {
        index == 0 || index.is_power_of_two()
    }

    /// return an iterator to the underlying vec
    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.0.iter()
    }

    /// Repair blocks based on their parity bits
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
                (0..16_u8)
                    .filter(|bit| !(strip_from_parity && Blocks::is_parity_bit(*bit)))
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
        let data = [0xff, 0xff, 0xff, 0xff];
        let blocks = Blocks::new(&data, false);

        assert_eq!(blocks.iter().as_slice().len(), 3);
    }

    #[test]
    fn new_passed_bytes_and_raw_leaves_bits_untouched() {
        let data = [0xff, 0xff, 0xff, 0xff];
        let blocks = Blocks::new(&data, true);

        assert_eq!(blocks.iter().as_slice().len(), 2);
    }

    #[test]
    fn to_string_returns_stringified_version_of_original_input() {
        let data = "some bytes";
        let blocks = Blocks::new(data.as_bytes(), false);

        assert_eq!(blocks.to_string(), data);
    }

    #[test]
    fn to_byte_vec_returns_raw_vec_of_u8() {
        let data = [0xff, 0xff];
        let blocks = Blocks::new(&data, true);

        assert_eq!(blocks.to_byte_vec(), vec![0xff, 0xff])
    }

    #[test]
    fn to_byte_vec_returns_prepared_vec_of_u8() {
        let data = [0xff, 0xff];
        let blocks = Blocks::new(&data, false);

        assert_eq!(blocks.to_byte_vec(), vec![0x7f, 0xff, 0x3f, 0xc0])
    }

    #[test]
    fn back_to_back() {
        // encode
        let data = "some bytes";
        let blocks = Blocks::new(data.as_bytes(), false);
        let raw_bytes = blocks.to_byte_vec();

        // decode
        let decode_blocks = Blocks::new(raw_bytes.as_slice(), true);
        assert_eq!(decode_blocks.to_string(), data);
    }
}
