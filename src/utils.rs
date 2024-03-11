use crate::state::State;

pub fn keccak(capacity: usize, size: usize, input: Vec<u8>) -> String {
    let r = 1600 - capacity;
    let block_size = r / 8;

    let p = pad10star1(input, block_size);

    let mut state = State::new();

    for block in p.chunks(block_size) {
        state.absorb(block.to_vec());
    }

    bytes_to_hex(state.squeeze(r, size))
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
