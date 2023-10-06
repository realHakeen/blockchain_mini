
use secp256k1::{PublicKey,Message,hashes::Hash,hashes::sha256};
use primitive_types::*;



#[derive(Debug,Clone, Copy)]
pub struct PeerID(H256);
// 如何分配peerid？为了MVP原则，我们暂时使用public key进行hash，成为独一无二的peerid
// 我们维护一个peerid list，


impl PeerID {
    pub fn new(_public_key:PublicKey)->Self{
        let mut public_key_bytes = _public_key.serialize();
        let hash_of_bytes = sha256::Hash::hash(&public_key_bytes);
        PeerID(H256(hash_of_bytes.to_byte_array()))
    }
}


mod tests{
    use secp256k1::{PublicKey,SecretKey, rand};

    use super::PeerID;
    # [test]
    fn test_peerid_hash(){
        let secp = secp256k1::Secp256k1::new();
        let secrate_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secrate_key);
        let my_peerid = PeerID::new(public_key);
        eprint!("{:?}",my_peerid.0);
    }
}

