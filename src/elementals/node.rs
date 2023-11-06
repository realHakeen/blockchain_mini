
use secp256k1::rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use crate::elementals::peerid::PeerID;
use crate::Blockchain::blockchain::Blockchain;
use crate::TransactionPool::transactionpool::{TransactionPool, self};
use super::address::Address;
use super::block::Block;
use super::peerid;
use super::transaction::Transaction;
use crate::Networking::discv::NodeDiscvService;

// Node是我们对整个网络对象的抽象
//一个节点需要私钥、地址、peerid进行寻找(peerid，我们自己定义，libp2p我们暂时不使用，底层的交流我们自己实现)
//初始化本地blockchain和txpool
//Node接入Networking模块，获取其它节点的数据

#[derive(Debug,Clone)]
pub struct Node{
    pub secrate_key:SecretKey,
    pub public_key:PublicKey,
    pub address:Address,
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
        let address:Address;

        // 其实可以使用match 匹配更多情况 但是我们MVP
        if _secrate_key == None{
            secrate_key = SecretKey::new(&mut rand::thread_rng());
            public_key = PublicKey::from_secret_key(&secp, &secrate_key);
            address = Address::get_address_from(public_key);
            peer_id = PeerID::new(public_key);
            
        }else {
            secrate_key = _secrate_key.unwrap();
            public_key = PublicKey::from_secret_key(&secp, &_secrate_key.unwrap());
            address = Address::get_address_from(public_key);
            peer_id = PeerID::new(public_key);
        }
        let blockchain:Blockchain = Blockchain::new();
        let transactionpool:TransactionPool = TransactionPool::new();
        
        Node { secrate_key: secrate_key, public_key: public_key,address:address,peer_id: peer_id, blockchain: blockchain, transaction_pool: transactionpool }
        
    }
    //查找其它节点的具体工作是由Networking模块下的discv协议完成
    pub fn find_other_node(&self){
        let node_discv_service = NodeDiscvService::bind(self.peer_id);
        node_discv_service.find_other_node();
    }
    //找到其它节点，维护好nodelist之后，需要从其他节点处取得对应的信息，这部分的实现在networking中
    //node节点发起一笔交易，该部分在node层面进行调用，在transactionpool进行交易校验和存入
    pub fn send_a_new_transaction(&mut self,_transaction:&Transaction)->bool{
        let success = self.transaction_pool.add_tx_to_transaction_pool(&_transaction);
        success
    }
    //把transaction存入node对应的transactionpool之后，miner负责打包交易，并且形成pre-block
    pub fn get_pre_block()->Block{
        todo!()
    }
    
    //miner负责调用consensus模块进行block确认，miner返回saledBlock给node
    //node调用将该saledBlock调用Networking模块在网络中广播
    //每12s，都会自动进行区块打包，这里要分线程
    
    
}





mod tests{
    use std::ptr::eq;

    use libp2p::identity::PublicKey;
    use secp256k1::{SecretKey, rand};
    use crate::Account::account::Account;
    use crate::{elementals::transaction::Transaction, Account::account::ACCOUNTS};
    use crate::Networking::discv::NodeDiscvService;
    use primitive_types::{H256,U256};

    use super::Node;

    # [test]

    fn test_find_other_node(){
        let mut mynode = Node::new(None);
        let find_node_service = NodeDiscvService::bind(mynode.peer_id);
        eprint!("My Node Secretary_key is {:?},my peerid is {:?}",mynode.secrate_key,mynode.peer_id);
        eprint!("find the node list is {:?}",find_node_service.node_list);

    }
    # [test]
    fn test_send_a_new_transaction(){
        let mut mynode = Node::new(None);
        let one_transaction = Transaction::new(mynode.address,
            mynode.address,
            U256::from(10),
            U256::from(10),
            U256::from(10),
            vec![],
            H256::from_low_u64_be(0)
        );
        // 给账户加点钱呀 没钱怎么转账？？？？ 无语死了
        unsafe{
            ACCOUNTS.insert(mynode.address, Account::new(U256([100;4])));
        }
        let success = mynode.send_a_new_transaction(&one_transaction);
        assert_eq!(true,success);
        
    }

}
