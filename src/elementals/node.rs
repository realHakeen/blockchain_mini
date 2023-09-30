use std::{default, str::FromStr};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use crate::elementals::address::{Address,self, get_address, get_key_pair};
use primitive_types::*;
use crate::elementals::peerid::PeerID;
use crate::Blockchain::blockchain::Blockchain;

// Node是我们对整个网络对象的抽象
//一个节点需要私钥、地址、peerid进行寻找(peerid，我们自己定义，libp2p我们暂时不使用，底层的交流我们自己实现)
//初始化本地blockchain和txpool
//Node接入Networking模块，获取其它节点的数据

pub struct Node{
    secrate_key:SecretKey,
    pub public_key:PublicKey,
    pub peer_id:PeerID,
    pub blockchain:Blockchain,
}

// node基本职责：初始化blockchain以及txpool，接入networking模块
impl Node {
    pub fn new() -> Self{
        todo!()
    }
}