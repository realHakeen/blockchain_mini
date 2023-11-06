use std::fmt::Display;

use primitive_types;
use secp256k1::Error;
use secp256k1::Message;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;
use secp256k1::ecdsa;
use secp256k1::ecdsa::Signature;




pub fn sign_a_signature<T:Display>(_secret_key:&SecretKey,_string:T)->Option<Signature>{
    let string_lize =  _string.to_string();
    let string_lize_as_u8 = string_lize.as_bytes();
    //对齐

    let secp = Secp256k1::new();
    let msg = Message::from_slice(&string_lize_as_u8);
    let signature:Signature;
    match msg {
        Ok(msg) => {
            signature = secp.sign_ecdsa(&msg, &_secret_key);
            Some(signature)
        },
        Err(err) => {
            println!("sorry ur signature is failed the error is {}",err);
            None
        }
    }
    
}