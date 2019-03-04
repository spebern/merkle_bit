#[cfg(feature = "use_keccak")]
use tiny_keccak::Keccak;

#[cfg(feature = "use_keccak")]
pub struct KeccakHasher(Keccak);

#[cfg(feature = "use_keccak")]
impl crate::traits::Hasher for KeccakHasher {
    type HashType = Self;
    type HashResultType = Vec<u8>;

    fn new(_size: usize) -> Self {
        let hasher = Keccak::new_keccak256();
        Self(hasher)
    }
    fn update(&mut self, data: &[u8]) { self.0.update(data); }
    fn finalize(self) -> Self::HashResultType {
        let mut res = vec![0u8; 32];
        self.0.finalize(&mut res);
        res
    }
}