use std::fmt;

pub struct Block {
    pub data: u16
}

impl From<u16> for Block {
    fn from(data: u16) -> Self {
        let mut block = Block {
            data: data
        };

        block.assign_parity(&block.parity());

        block
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
            false => "0"
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
    pub fn parity(&self) -> u8 {
        (0..16)
            .filter(|bit| &self.data >> 15 - bit & 0b1 > 0)
            .fold(0u8, |acc, bit| acc ^ bit)
    }

    pub fn assign_parity(&mut self, parity: &u8){
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

    pub fn repair(&mut self) {
        match self.parity() {
            0 => (),
            err => {
                self.data ^= 0b1 << 15 - err as u16;
            },
        }
    }

    pub fn flip_random_bit(&mut self) {
        self.data ^= 0b1 << rand::random::<u8>() % 15
    }
}
