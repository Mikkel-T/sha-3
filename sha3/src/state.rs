/// Offsets for the rho step mapping as defined in table 2 of FIPS 202, the rho offsets have been minimized by doing "mod 64" on all of them.
const RHO_OFFSETS: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

/// Round constants for the iota step mapping retrieved from [https://keccak.team/keccak_specs_summary.html](https://keccak.team/keccak_specs_summary.html).
const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

pub struct State {
    state: [[u64; 5]; 5],
}

impl State {
    pub fn new() -> State {
        State { state: [[0; 5]; 5] }
    }

    /// Absorb a block of data into the state and apply the permutation.
    pub fn absorb(&mut self, data: Vec<u8>) {
        // Breaking the data up into 8 chunks with the size of a byte to get 64 bit words.
        for (i, chunk) in data.chunks(8).enumerate() {
            let mut word = 0;
            for (j, byte) in chunk.iter().enumerate() {
                // Byte 0-7 will be used like this: 7 || 6 || 5 || 4 || 3 || 2 || 1 || 0. So we are using little endian arrangement of the bytes where z=0 is the least significant bit of the first byte. This is in accordance with the standard where bit strings are reversed compared to the bytes they represent.
                // OR the byte into the word, and use the index to left shift it.
                word |= (*byte as u64) << (8 * j);
            }
            self.state[i % 5][i / 5] ^= word;
        }

        self.permute();
    }

    /// Output a string from a sponge state in accordance with step 7-10 of section 5 of FIPS 202. First get a "rate" size block from the state, add it to the output and do a permutation until the wanted size is achieved.
    pub fn squeeze(&mut self, rate: usize, size: usize) -> Vec<u8> {
        let mut out = Vec::new();

        while out.len() < size / 8 {
            // Truncate the state to the size of the rate, dividing by 64 since a word is 64 bits (rate is always divisible by 64).
            for i in 0..(rate / 64) {
                let word = self.state[i % 5][i / 5];
                // LE because we save the state in little endian.
                out.extend_from_slice(&word.to_le_bytes());
            }

            self.permute();
        }

        // Only output the needed size
        out[0..size / 8].to_vec()
    }

    /// Do a full keccak permutation (24 rounds)
    fn permute(&mut self) {
        for i in 0..24 {
            self.round(i);
        }
    }

    /// Do a round of the keccak step mappings with the round input being the number used in the iota step.
    fn round(&mut self, round: usize) {
        self.theta();
        self.rho();
        self.pi();
        self.chi();
        self.iota(round);
    }

    /// The theta step mapping as defined in section 3.2.1 of FIPS 202.
    #[allow(clippy::needless_range_loop)]
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
            // Add 4 instead of subtracting 1 to avoid overflow. "x + 4 mod 5" is the same as "x - 1 mod 5".
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }

        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] ^= d[x];
            }
        }
    }

    /// The rho step mapping as defined in section 3.2.2 of FIPS 202, but with the offsets precalculated.
    #[allow(clippy::needless_range_loop)]
    fn rho(&mut self) {
        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = self.state[x][y].rotate_left(RHO_OFFSETS[x][y]);
            }
        }
    }

    /// The pi step mapping as defined in section 3.2.3 of FIPS 202.
    fn pi(&mut self) {
        // Use a tmp state as to not mess with the current state while working on it.
        let tmp = self.state;

        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = tmp[(x + 3 * y) % 5][x];
            }
        }
    }

    /// The chi step mapping as defined in section 3.2.4 of FIPS 202.
    fn chi(&mut self) {
        let tmp = self.state;

        for x in 0..5 {
            for y in 0..5 {
                // XOR 1 for a bit is the same as NOT.
                self.state[x][y] = tmp[x][y] ^ ((!tmp[(x + 1) % 5][y]) & tmp[(x + 2) % 5][y]);
            }
        }
    }

    /// The iota step mapping as defined in section 3.2.5 of FIPS 202, but with the round constants precalculated.
    fn iota(&mut self, round: usize) {
        self.state[0][0] ^= ROUND_CONSTANTS[round];
    }
}
