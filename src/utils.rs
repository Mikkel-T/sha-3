use crate::state::State;

/// Implementation of the keccak function to be used for SHA-3 and SHAKE with the capacity, output size and input vector as input. The capacity is used to determine the block size.
pub fn keccak(capacity: usize, size: usize, input: Vec<u8>) -> String {
    // Rate is defined as 1600 - capacity in the keccak algorithm used by SHA-3 and SHAKE.
    let r = 1600 - capacity;

    // Since capacity and rate is in bits, we need to change it to bytes for further use.
    let block_size = r / 8;

    // Padding using the pad10*1 algorithm.
    let p = pad10star1(input, block_size);

    let mut state = State::new();

    // For each of the blocks, we "absorb" the block into the state and do the keccak permutations.
    for block in p.chunks(block_size) {
        state.absorb(block.to_vec());
    }

    bytes_to_hex(state.squeeze(r, size))
}

/// Padding according to appendix B.2 of FIPS 202. The domain separator is added in the SHA-3 and SHAKE functions.
fn pad10star1(m: Vec<u8>, block_size: usize) -> Vec<u8> {
    let mut p = m;

    // The domain separator is added, so we just start adding 0-bytes until the length is a divisor of the block size. If it already is, nothing happens.
    while p.len() % block_size != 0 {
        p.push(0x00);
    }

    // XOR the last byte with 0x80. This way, a 0-byte will become 0x80 while a domain separator will become the intended byte.
    let i = p.len() - 1;
    p[i] ^= 0x80;

    p
}

/// Simple method of getting a hex string from a vec of bytes
fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut s = String::new();

    for byte in bytes {
        // Use {:02x} to always ensure two characters, even for 0-bytes.
        s.push_str(&format!("{:02x}", byte));
    }

    s
}
