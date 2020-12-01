pub struct BitVec {
    source: Vec<u8>,
    index: usize,
}

impl BitVec {
    pub fn from_bytes(data: &[u8]) -> Self {
        BitVec {
            source: Vec::from(data),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.source.len()
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

#[cfg(test)]
mod bitvec_tests {
    use super::BitVec;

    #[test]
    fn from_bytes_converts_to_vec_of_bools() {
        let mut bitvec = BitVec::from_bytes("a".as_bytes());

        // a = 01100001
        let expected = [false, true, true, false, false, false, false, true];
        for expected_bit in expected.iter() {
            assert_eq!(*expected_bit, bitvec.next().unwrap())
        }
    }

    #[test]
    fn iterator_iterates_correct_amount() {
        let mut bitvec = BitVec::from_bytes(&[255, 255, 255]).peekable();

        // we can iterate 3 * 8 = 24 times and fully exhaust the iterator
        for _ in 0..(3 * 8) {
            assert!(bitvec.peek().is_some());
            assert!(bitvec.next().is_some());
        }

        // but then we're at the end
        assert_eq!(bitvec.next(), None);
    }
}
