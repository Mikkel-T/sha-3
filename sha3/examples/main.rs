use sha3::{sha3_224, sha3_256, sha3_384, sha3_512, shake128, shake256};

fn main() {
    println!("{}", sha3_224(""));
    println!("{}", sha3_256(""));
    println!("{}", sha3_384(""));
    println!("{}", sha3_512(""));
    println!("{}", shake128("", 256));
    println!("{}", shake256("", 512));
}
