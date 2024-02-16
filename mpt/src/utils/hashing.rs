use sha2::{Digest, Sha256};
use crate::Hash;
pub(crate) fn to_hash<T:AsRef<[u8]>>(s:&T)->Hash{
    let mut hasher = Sha256::new();
    hasher.update(s);
    hasher.finalize().into()
}
