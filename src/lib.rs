#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]
extern crate hex;

/// The number of rounds to use in the knot hash function
const KNOT_ROUNDS: usize = 64;

/// Used for creating knot hashes
/// Preserves state for performing rounds
pub struct KnotHasher {
    pub bytes: [u8; 256],
    cur_pos: usize,
    skip_size: usize,
}
impl KnotHasher {
    /// Creates a new knot hasher
    pub fn new() -> KnotHasher {
        // Initialize the bytes to 0-255
        let mut bytes: [u8; 256] = [0; 256];
        for index in 0..=255 {
            bytes[index as usize] = index;
        }
        // Return a knothasher with the values initialized
        KnotHasher {
            bytes: bytes,
            cur_pos: 0,
            skip_size: 0,
        }
    }
    /// Resets the knot hasher's bytes and internal round state
    pub fn reset(&mut self) {
        *self = KnotHasher::new();
    }
    /// Performs some rounds with the knot hasher
    pub fn do_rounds(&mut self, lengths: &[u8], num_rounds: usize) {
        for _ in 0..num_rounds {
            for length in lengths {
                // Reverse the desired length
                // Middle will be ignored
                for pos in 0..((length / 2) as usize) {
                    // Get each position and its mirror
                    let pos_x = (self.cur_pos + pos) % 256;
                    let pos_y = (self.cur_pos + (*length as usize - 1) - pos) % 256;
                    // Perform the swap
                    let temp: u8 = self.bytes[pos_x];
                    self.bytes[pos_x] = self.bytes[pos_y];
                    self.bytes[pos_y] = temp;
                }
                self.cur_pos += *length as usize + self.skip_size;
                self.skip_size += 1;
            }
        }
    }
    /// Calculates the dense knot hash for a string
    pub fn dense_knot_hash(&mut self, input: &str) -> [u8; 16] {
        // Reset self so the hash is calculated correctly
        self.reset();
        // Used to store the bytes. Use input length + 5 as an estimate of needed capacity
        let mut data: Vec<u8> = Vec::with_capacity(input.len() + 5);
        // Convert string to ascii and add values to the vector
        data.extend_from_slice(input.as_bytes());
        // Add the extra values
        data.extend_from_slice(&[17, 31, 73, 47, 23]);
        // Perform the hash rounds
        self.do_rounds(&data, KNOT_ROUNDS);
        // Chunk the output and create a dense hash
        let mut dense_hash: [u8; 16] = [0; 16];
        for chunk_index in 0..16 {
            dense_hash[chunk_index] = self.bytes[(chunk_index * 16)..((chunk_index * 16) + 16)].iter().fold(
                0,
                |x, y| x ^ y,
            )
        }
        dense_hash
    }
    /// Calculates the knot hash for a string, represented as hexadecimal
    pub fn knot_hash(&mut self, input: &str) -> String {
        hex::encode(self.dense_knot_hash(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_knot_hash() {
        // Build a hasher
        let mut knot_hasher: KnotHasher = KnotHasher::new();
        // Test the given test cases
        assert_eq!(knot_hasher.knot_hash(""),         "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hasher.knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hasher.knot_hash("1,2,3"),    "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hasher.knot_hash("1,2,4"),    "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
