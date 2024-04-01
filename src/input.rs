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
