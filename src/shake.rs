use crate::input::Input;
use crate::utils::keccak;

pub fn shake<T: Input>(capacity: usize, input: T, size: usize) -> String {
    match capacity {
        128 | 256 => (),
        _ => panic!("Invalid capacity"),
    }

    if size % 8 != 0 {
        panic!("Invalid size");
    }

    let mut m = input.convert().to_vec();
    m.push(0x1F); // Add the SHAKE domain separator and first padding bit
    keccak(capacity * 2, size, m)
}

pub fn shake128<T: Input>(input: T, size: usize) -> String {
    shake(128, input, size)
}

pub fn shake256<T: Input>(input: T, size: usize) -> String {
    shake(256, input, size)
}

#[cfg(test)]
mod tests {
    use super::*;

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
