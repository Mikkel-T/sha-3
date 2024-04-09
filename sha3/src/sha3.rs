use crate::input::Input;
use crate::utils::keccak;

/// The SHA-3 algorithm. Here, the size is the final length of the output in bits (has to be one of the standard sizes 224, 256, 384 or 512) and the input can be of any type that implements the Input trait.
pub fn sha3<T: Input>(size: usize, input: T) -> String {
    match size {
        224 | 256 | 384 | 512 => (),
        _ => panic!("Invalid size"),
    }

    let mut m = input.convert().to_vec();
    m.push(0x06); // Add the SHA-3 domain separator and first padding bit
    keccak(size * 2, size, m)
}

/// Shortcut for the SHA3-224 function, the input can be of any type that implements the Input trait.
pub fn sha3_224<T: Input>(input: T) -> String {
    sha3(224, input)
}

/// Shortcut for the SHA3-256 function, the input can be of any type that implements the Input trait.
pub fn sha3_256<T: Input>(input: T) -> String {
    sha3(256, input)
}

/// Shortcut for the SHA3-384 function, the input can be of any type that implements the Input trait.
pub fn sha3_384<T: Input>(input: T) -> String {
    sha3(384, input)
}

/// Shortcut for the SHA3-512 function, the input can be of any type that implements the Input trait.
pub fn sha3_512<T: Input>(input: T) -> String {
    sha3(512, input)
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
}
