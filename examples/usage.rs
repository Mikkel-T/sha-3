use sha3::{sha3, sha3_512, shake128};

fn main() {
    println!("{}", sha3_512("e"));
    println!("{}", sha3(224, "e"));
    println!("{}", shake128("e", 256));
}
