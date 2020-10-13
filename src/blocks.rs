#![allow(dead_code)]
use rand;

use std::iter::FromIterator;

use super::block::Block;
use super::bitvec::BitVec;

pub struct Blocks(Vec<Block>);
impl From<&str> for Blocks {
    fn from(data: &str) -> Blocks {
        Blocks::new(data.as_bytes())
    }
}

impl Blocks {
    pub fn new(data: &[u8]) -> Self {
        // allocate block buffer
        let mut blocks = Vec::new();

        // make the data ierable per byte
        let mut bits = BitVec::from_bytes(data).peekable();

        // iterate until theres no more bits
        while bits.peek().is_some() {
            let block = (0..15usize).rev()
                .filter(|bit|  !usize::is_power_of_two(15 - bit))
                .fold(0u16, |block, bit| {
                    match bits.next() {
                        Some(true) =>  block | 0b1 << bit,
                        Some(false) => block | 0b0 << bit,
                        None => block
                    }
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

    pub fn to_string(&self) -> String {
        // read all blocks and store as a vec of bools
        let bits:Vec<bool> = self.iter().flat_map(|block| {
            (1..16usize)
                .filter(|bit| !bit.is_power_of_two())
                .map(move |bit| {
                    block.data >> 15 - bit & 0b1 > 0
                })
        }).collect();

        // chunk per 8 bits to turn into bytes
        let chars : Vec<char> = bits
            .chunks(8)
            .filter(|chunk| chunk.len() >= 8)
            .map(|chunk| {
                let byte = chunk
                    .iter()
                    .enumerate()
                    .fold(0u8, |byte, (idx, bit)| {
                        match bit {
                            true =>  byte | 0b1 << 7 - idx,
                            false => byte | 0b0 << 7 - idx,
                        }
                    });

                char::from(byte)
            })
        .collect();

        String::from_iter(chars)
    }
}

