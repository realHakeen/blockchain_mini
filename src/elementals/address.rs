extern crate secp256k1;
use std::clone;

use secp256k1::hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use primitive_types::{U256,H256};

/// 我们的Adress类型是用来标记单一账户地址的，privatekey是通过secp256k1生成；
/// Adress是通过ECDSA(keccak256)进行生成的。
/// 因此我们要开始细致研究椭圆曲线算法。(docs/research/ECDSA.md)
/// 根据研究，我们首先用secp256k1生成了secretKey和publicKey。
/// 用Keccak256压缩publickey，返回最后二十bytes作为地址
///
///
///
///



pub fn get_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    println!(
        "SecrectKey : {:?}/n PublicKey : {:?}",
        secret_key, public_key
    );
    (secret_key, public_key)
}

//这里需要注意使用Vec可以避免内存拷贝的开销，因为基于堆的拷贝，是浅拷贝，
//地址是public_key serialize后的前20字节
pub fn get_address(public_key: PublicKey) -> Vec<u8> {
    let serialize_address: [u8; 33] = public_key.serialize();
    let address = serialize_address[serialize_address.len() - 20..].to_vec();
    address
}
//测试用户输入的srcrate_key是否符合要求，然后从私钥生成公钥

pub fn get_public_key(secrate_key:SecretKey) -> PublicKey{
    let secp = Secp256k1::new();
    PublicKey::from_secret_key(&secp,&secrate_key)
}
