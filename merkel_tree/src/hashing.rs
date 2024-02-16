use sha2::{Digest, Sha256};

pub(crate) fn to_hash<T:AsRef<[u8]>>(s:&T)->[u8;32]{
    let mut hasher = Sha256::new();
    hasher.update(s);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests{
    use crate::hashing::to_hash;

    #[test]
    fn sizeof_hash(){
        println!("{}",to_hash(&String::from("testing")).len());
    }
}