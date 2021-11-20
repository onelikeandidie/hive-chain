use crate::Hash;

pub trait Hashable {
    fn bytes(&self) -> Hash;
    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}