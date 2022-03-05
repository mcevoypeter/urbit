/// Serialize into a byte buffer.
pub trait Jam {
    fn jam(self) -> Vec<u8>;
}

/// Deserialize from a byte buffer.
pub trait Cue {
    fn cue(bytes: Vec<u8>) -> Self;
}
