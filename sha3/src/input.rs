/// The trait implemented to handle input for the hashing algorithms.
pub trait Input {
    /// Convert a type to the wanted input form.
    fn convert(&self) -> &[u8];
}

impl Input for &str {
    fn convert(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Input for String {
    fn convert(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Input for &[u8] {
    fn convert(&self) -> &[u8] {
        self
    }
}

impl Input for Vec<u8> {
    fn convert(&self) -> &[u8] {
        self
    }
}

impl Input for &String {
    fn convert(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_input() {
        let input: &str = "Hello, world!";
        let converted = input.convert();
        assert_eq!(converted, input.as_bytes());
    }

    #[test]
    fn test_string_input() {
        let input: String = String::from("Hello, world!");
        let converted = input.convert();
        assert_eq!(converted, input.as_bytes());
    }

    #[test]
    fn test_slice_input() {
        let input: &[u8] = &[1, 2, 3, 4, 5];
        let converted = input.convert();
        assert_eq!(converted, input);
    }

    #[test]
    fn test_vec_input() {
        let input: Vec<u8> = vec![1, 2, 3, 4, 5];
        let converted = input.convert();
        assert_eq!(converted, input.as_slice());
    }

    #[test]
    fn test_string_ref_input() {
        let input: &String = &String::from("Hello, world!");
        let converted = input.convert();
        assert_eq!(converted, input.as_bytes());
    }
}
