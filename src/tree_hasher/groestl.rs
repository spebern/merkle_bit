use groestl::{Digest, Groestl256};

use crate::constants::KEY_LEN;

pub struct GroestlHasher(Groestl256);

impl crate::traits::Hasher for GroestlHasher {
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        let hasher = Groestl256::new();
        Self(hasher)
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.input(data);
    }

    #[inline]
    fn finalize(self) -> [u8; KEY_LEN] {
        let mut finalized = [0; KEY_LEN];
        let result = self.0.result();
        finalized.copy_from_slice(&result);
        finalized
    }
}
