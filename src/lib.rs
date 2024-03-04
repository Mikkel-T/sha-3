mod consts;
pub mod input;

use consts::{RHO_OFFSETS, ROUND_CONSTANTS};
use input::Input;

pub fn sha3<T: Input>(size: usize, input: T) -> String {
    if size % 8 != 0 {
        panic!("Invalid size");
    }

    let c = size * 2;
    let r = 1600 - c;
    let block_size = r / 8;

    let mut m = input.convert().to_vec();
    m.push(0x06); // Add the SHA-3 domain separator and first padding bit
    let p = pad10star1(m, block_size);

    let mut state = State::new();

    for block in p.chunks(block_size) {
        state.absorb(block.to_vec());
    }

    bytes_to_hex(state.squeeze(r, size))
}

pub fn sha3_512<T: Input>(input: T) -> String {
    sha3(512, input)
}

pub fn sha3_384<T: Input>(input: T) -> String {
    sha3(384, input)
}

pub fn sha3_256<T: Input>(input: T) -> String {
    sha3(256, input)
}

pub fn sha3_224<T: Input>(input: T) -> String {
    sha3(224, input)
}

pub fn shake<T: Input>(capacity: usize, input: T, size: usize) -> String {
    if capacity % 8 != 0 {
        panic!("Invalid capacity");
    }

    if size % 8 != 0 {
        panic!("Invalid size");
    }

    let c = capacity * 2;
    let r = 1600 - c;
    let block_size = r / 8;

    let mut m = input.convert().to_vec();
    m.push(0x1F); // Add the SHAKE domain separator and first padding bit
    let p = pad10star1(m, block_size);

    let mut state = State::new();

    for block in p.chunks(block_size) {
        state.absorb(block.to_vec());
    }

    bytes_to_hex(state.squeeze(r, size))
}

pub fn shake128<T: Input>(input: T, size: usize) -> String {
    shake(128, input, size)
}

pub fn shake256<T: Input>(input: T, size: usize) -> String {
    shake(256, input, size)
}

#[derive(Clone)]
struct State {
    state: [[u64; 5]; 5],
}

impl State {
    fn new() -> State {
        State { state: [[0; 5]; 5] }
    }

    // Absorb a block of data into the state and apply the permutation
    pub fn absorb(&mut self, data: Vec<u8>) {
        // Breaking the data up into 8 byte chunks to get 64 bit words
        for (i, chunk) in data.chunks(8).enumerate() {
            let mut word = 0;
            for (j, byte) in chunk.iter().enumerate() {
                // Byte 0-7 will be used like this: 7 || 6 || 5 || 4 || 3 || 2 || 1 || 0, so little endian where z=0 is the least significant bit
                word |= (*byte as u64) << (8 * j);
            }
            self.state[i % 5][i / 5] ^= word;
        }

        self.permute();
    }

    fn permute(&mut self) {
        for i in 0..24 {
            self.round(i);
        }
    }

    fn round(&mut self, round: usize) {
        self.theta();
        self.rho();
        self.pi();
        self.chi();
        self.iota(round);
    }

    pub fn squeeze(&mut self, rate: usize, size: usize) -> Vec<u8> {
        let mut out = Vec::new();

        while out.len() < size / 8 {
            // Truncate the state to the size of the rate
            for i in 0..(rate / 64) {
                let word = self.state[i % 5][i / 5];
                // LE because we save the state in little endian
                out.extend_from_slice(&word.to_le_bytes());
            }

            self.permute();
        }

        out[0..size / 8].to_vec()
    }

    fn theta(&mut self) {
        let mut c = [0; 5];

        for x in 0..5 {
            c[x] = self.state[x][0]
                ^ self.state[x][1]
                ^ self.state[x][2]
                ^ self.state[x][3]
                ^ self.state[x][4];
        }

        let mut d = [0; 5];

        for x in 0..5 {
            // Add 4 instead of 1 to avoid overflow
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }

        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] ^= d[x];
            }
        }
    }

    fn rho(&mut self) {
        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = self.state[x][y].rotate_left(RHO_OFFSETS[x][y]);
            }
        }
    }

    fn pi(&mut self) {
        let tmp = self.state.clone();

        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = tmp[(x + 3 * y) % 5][x];
            }
        }
    }

    fn chi(&mut self) {
        let tmp = self.state.clone();

        for x in 0..5 {
            for y in 0..5 {
                // XOR 1 is the same as NOT
                self.state[x][y] = tmp[x][y] ^ ((!tmp[(x + 1) % 5][y]) & tmp[(x + 2) % 5][y]);
            }
        }
    }

    fn iota(&mut self, round: usize) {
        self.state[0][0] ^= ROUND_CONSTANTS[round];
    }
}

// Padding according to appendix B.2 of FIPS 202
fn pad10star1(m: Vec<u8>, block_size: usize) -> Vec<u8> {
    let mut p = m;

    while p.len() % block_size != 0 {
        p.push(0x00);
    }

    let i = p.len() - 1;
    p[i] ^= 0x80;

    p
}

fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut s = String::new();

    for byte in bytes {
        s.push_str(&format!("{:02x}", byte));
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha3_224() {
        assert_eq!(
            sha3_224(""),
            "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"
        );

        assert_eq!(
            sha3_224("The quick brown fox jumps over the lazy dog"),
            "d15dadceaa4d5d7bb3b48f446421d542e08ad8887305e28d58335795"
        );
    }

    #[test]
    fn test_sha3_256() {
        assert_eq!(
            sha3_256(""),
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
        );

        assert_eq!(
            sha3_256("The quick brown fox jumps over the lazy dog"),
            "69070dda01975c8c120c3aada1b282394e7f032fa9cf32f4cb2259a0897dfc04"
        );
    }

    #[test]
    fn test_sha3_384() {
        assert_eq!(
            sha3_384(""), 
            "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004"
        );

        assert_eq!(
            sha3_384("The quick brown fox jumps over the lazy dog"),
            "7063465e08a93bce31cd89d2e3ca8f602498696e253592ed26f07bf7e703cf328581e1471a7ba7ab119b1a9ebdf8be41"
        );
    }

    #[test]
    fn test_sha3_512() {
        assert_eq!(
            sha3_512(""),
            "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
        );

        assert_eq!(
            sha3_512("The quick brown fox jumps over the lazy dog"),
            "01dedd5de4ef14642445ba5f5b97c15e47b9ad931326e4b0727cd94cefc44fff23f07bf543139939b49128caf436dc1bdee54fcb24023a08d9403f9b4bf0d450"
        );
    }

    #[test]
    fn test_shake128() {
        assert_eq!(
            shake128("", 256),
            "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26"
        );

        assert_eq!(
            shake128("The quick brown fox jumps over the lazy dog", 256),
            "f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae17db0f8cfeed0e3e66e"
        );
    }

    #[test]
    fn test_shake256() {
        assert_eq!(
            shake256("", 512),
            "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"
        );

        assert_eq!(
            shake256("The quick brown fox jumps over the lazy dog", 512),
            "2f671343d9b2e1604dc9dcf0753e5fe15c7c64a0d283cbbf722d411a0e36f6ca1d01d1369a23539cd80f7c054b6e5daf9c962cad5b8ed5bd11998b40d5734442"
        );
    }
}
