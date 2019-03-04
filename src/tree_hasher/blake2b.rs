#[cfg(feature = "use_blake2b")]
use std::cmp::Ordering;

#[cfg(feature = "use_blake2b")]
use blake2_rfc;

#[cfg(feature = "use_blake2b")]
#[derive(Clone)]
pub struct Blake2bHasher(blake2_rfc::blake2b::Blake2b);

#[cfg(feature = "use_blake2b")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blake2bHashResult(blake2_rfc::blake2b::Blake2bResult);

#[cfg(feature = "use_blake2b")]
impl PartialOrd for Blake2bHashResult {
    fn partial_cmp(&self, other: &Blake2bHashResult) -> Option<Ordering> {
        Some(self.0.as_ref().cmp(&other.0.as_ref()))
    }
}

#[cfg(feature = "use_blake2b")]
impl AsRef<[u8]> for Blake2bHashResult {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(feature = "use_blake2b")]
impl crate::traits::Hasher for Blake2bHasher {
    type HashType = Self;
    type HashResultType = Blake2bHashResult;

    fn new(size: usize) -> Self {
        let hasher = blake2_rfc::blake2b::Blake2b::new(size);
        Self(hasher)
    }
    fn update(&mut self, data: &[u8]) { self.0.update(data); }
    fn finalize(self) -> Self::HashResultType { Blake2bHashResult(self.0.finalize()) }
}