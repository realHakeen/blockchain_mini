
use secp256k1::rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use crate::elementals::peerid::PeerID;
use crate::Blockchain::blockchain::Blockchain;
use crate::TransactionPool::transactionpool::{TransactionPool, self};
use super::peerid;

// Node是我们对整个网络对象的抽象
//一个节点需要私钥、地址、peerid进行寻找(peerid，我们自己定义，libp2p我们暂时不使用，底层的交流我们自己实现)
//初始化本地blockchain和txpool
//Node接入Networking模块，获取其它节点的数据

pub struct Node{
    secrate_key:SecretKey,
    pub public_key:PublicKey,
    //关于peerid如何分配？
    pub peer_id:PeerID,
    pub blockchain:Blockchain,
    pub transaction_pool:TransactionPool,
}

// node基本职责：初始化blockchain以及txpool，接入networking模块，networking负责每个slot的具体事务
impl Node {
    pub fn new(_secrate_key:Option<SecretKey>) -> Self{
        let secp:Secp256k1<secp256k1::All> = Secp256k1::new();
        let secrate_key:SecretKey;
        let public_key:PublicKey;
        let peer_id:PeerID;
        let blockchain:Blockchain = Blockchain::new();
        let transactionpool:TransactionPool = TransactionPool::new();
        // 其实可以使用match 匹配更多情况 但是我们MVP
        if _secrate_key == None{
            secrate_key = SecretKey::new(&mut rand::thread_rng());
            public_key = PublicKey::from_secret_key(&secp, &secrate_key);
            peer_id = PeerID::new(public_key);
            
        }else {
            public_key = PublicKey::from_secret_key(&secp, &_secrate_key.unwrap());
            peer_id = PeerID::new(public_key);
        }
        
        Node { secrate_key: _secrate_key.unwrap(), public_key: public_key, peer_id: peer_id, blockchain: blockchain, transaction_pool: transactionpool }
        
    }
}




mod tests{
    use libp2p::identity::PublicKey;
    use secp256k1::{SecretKey, rand};

    use super::Node;

    # [test]
    fn test_node_new_func(){
        // panic "not implement"
        
        
    }
}
