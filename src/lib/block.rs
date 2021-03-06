use std::fmt;

pub struct Block {
    pub data: u16,
}

impl From<u16> for Block {
    fn from(data: u16) -> Self {
        Block { data: data }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in 0..16 {
            // for every four bits, start on new line
            if bit % 4 == 0 {
                write!(f, "\n")?;
            }

            // determine the value
            let value = match &self.data >> 15 - bit & 0b1 > 0 {
                true => "1",
                false => "0",
            };

            // color the parity bits different
            if usize::is_power_of_two(bit) {
                write!(f, "\x1b[1;96;127m{} \x1b[0m", value)?;
            } else {
                write!(f, "{} ", value)?;
            }
        }

        write!(f, "\n")
    }
}

impl Block {
    /// return parity bits for the given block
    pub fn parity(&self) -> u8 {
        (0..16)
            .filter(|bit| &self.data >> 15 - bit & 0b1 > 0)
            .fold(0u8, |acc, bit| acc ^ bit)
    }

    /// prepare the block by calculating and assigning its parity
    pub fn prepare(&mut self) {
        let parity = &self.parity();

        self.data = (0..4).rev().fold(self.data, |acc, bit| {
            let parity_bit = match parity >> bit & 0b1 > 0 {
                true => 0b1,
                false => 0b0,
            };

            acc | parity_bit << 15 - 2u8.pow(bit)
        });
    }

    /// repair the block based on its parity bits
    pub fn repair(&mut self) {
        match self.parity() {
            err if err > 0 => self.data ^= 0b1 << 15 - err as u16,
            _ => (),
        }
    }
}

#[cfg(test)]
mod block_test {
    use super::Block;

    #[test]
    fn flip_random_bit_flips_a_random_bit() {
        let data: u16 = 0b00010101001110011;
        let mut block = Block::from(data);
        let before_data = block.data;
        flip_random_bit(&mut block);
        let after_data = block.data;

        assert_ne!(before_data, after_data)
    }

    #[test]
    fn repair_repairs_broken_block() {
        const DATA_MASK: u16 = 0b0001011101111111;
        let data = rand::random::<u16>() & DATA_MASK;
        let mut block = Block::from(data);
        block.prepare();

        let original_data = block.data;
        flip_random_bit(&mut block);
        block.repair();

        assert_eq!(block.data, original_data)
    }

    fn flip_random_bit(block: &mut Block) {
        let which_random_bit = rand::random::<u8>() % 15;
        block.data ^= 0b1 << which_random_bit;
    }
}
