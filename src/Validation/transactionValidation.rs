use crate::elementals::transaction::{self, Transaction};
use secp256k1::hashes::Hash;
use secp256k1::{SecretKey, Message};


// 暂时不验证，默认所有的交易都是有效的
struct Validator{

}

impl Validator {
    pub fn sign_transaction(_secrate_key:SecretKey,_transaction:Transaction){
        let secrate_key = _secrate_key;
    }
    pub fn validate_transaction(){
        todo!()
    }
}