
use std::borrow::Borrow;
use std::time::SystemTime;

use primitive_types::{U256, H256};
use secp256k1::{SecretKey, PublicKey, KeyPair, Secp256k1,rand};

use crate::VM::transactionExecute::Executor;
use crate::Account::account::ACCOUNTS;
use crate::Validation::transactionValidation;
use crate::elementals::head::Header;
use crate::elementals::node::Node;
use crate::elementals::transaction::{Transaction, self};
use crate::elementals::block::Block;
use crate::elementals::address::Address;


/// miner负责存入交易，调用交易的校验，目前仅仅校验交易中，
/// 转移数量的合法性，validation调用aacount查看状态
/// miner负责从txpool中挑选gas price最高的transaction，形成block
/// miner角色维护的数据暂时无
# [derive(Debug,Clone)]
pub struct Miner{
    keypair:KeyPair,
    address:Address,
}



impl Miner {
    pub fn new(_keypair :Option<KeyPair>)->Self{
        if _keypair.is_none() {
            let secp = Secp256k1::new();
            let keypair = KeyPair::new(&secp, &mut rand::thread_rng());
            let address = Address::get_address_from(keypair.public_key());
            Miner{
                keypair:keypair,
                address:address,
            }
                
            }else{
              
                Miner{
                    keypair:_keypair.unwrap(),
                    address:Address::get_address_from(_keypair.unwrap().public_key()),
                }
                
            }
        }
    
    // pick transactions 是从transactionpool里面找到一些transactions,n的数量是miner自己设置的，node是不管的
    // 我们一般认为当n = 10 或者slot在12s时，会pick transactions 发送给evm执行交易（目前evm只支持转账），并且打包成pre-block
    // 
    // 方法
    fn pick_transactions(_node:&mut Node,n:usize)->Vec<Transaction>{
        _node.transaction_pool.get_transaction_in_fcfs(n)
    }
    // evm负责把我们pick的transactions执行
    fn build_block(&self,_node:&mut Node,n:usize) -> Block{
        //首先是VM执行交易，然后全部成功的话，就打包成一个block
        let mut binding = Miner::pick_transactions(_node, n);
        let transactions = binding.iter_mut();
        let mut gas_used:U256 = U256([0;4]);
        for i in transactions {
            let success = Executor::execution_transaction(i);
            if success == false {
                gas_used += i.base_gas + i.clone().priority_fee;
            }else {
                continue;
            }
        }
        // miner形成一个pre-block
        // miner从node的blockchain中获得parent_block
        let parent_block = _node.blockchain.get_latest_block_H256();
        let timestamp = SystemTime::now();        
        let header = Header::new(parent_block, timestamp, gas_used);
        let block_number = _node.blockchain.get_latest_block().block_number;
        // transaction_root是H256，transaction_root的原理是把所有的transaction都作为hash，形成一个hash tree
        // 在pre-block是不写transaction root进入block内的，这个是作为校验存在
        
        let transaction_root: H256 = H256([0;32]);
        let miner_adderss =  self.address;
        let pre_block:Block = Block::new(header, block_number, transaction_root, miner_adderss);
        pre_block
        
        // 获取parent_block，要从哪里获得？
        

        //然后这个block一旦被consensus网络达成共识，那么即可成为block
        //在以太坊中，所有的交易不会成为

    }
    
}



mod tests{

    use primitive_types::{U256, H256};

    use crate::Account::account::{ACCOUNTS, Account};
    use crate::Miner::miner::Miner;
    use crate::elementals::address::Address;
    use crate::elementals::node::Node;
    use crate::elementals::transaction::Transaction;
    

    

    # [test]
    fn test_build_block<'a>(){
        //搞两个节点
        let mut node = Node::new(None);
        let mut node_second = Node::new(None);
        eprintln!("The node first address is {}, the node second address is {}",node.address,node_second.address);
        // 我们暂时不实现sign_transaction，因为实际上不是H256格式，后续需要verify，外貌引入了secp256k1::ecdsa库
        // 那么transaction格式中sig字段应该改为Signature
        
        // 首先给两个node充值点代币
        unsafe { ACCOUNTS.insert(node.address, Account::new(U256::from(101))) };
        unsafe { ACCOUNTS.insert(node_second.address, Account::new(U256::from(102))) };

        eprintln!("Top up {} token for node ,and {} token for node_second",
        unsafe {
            ACCOUNTS.get(&node.address).unwrap().balance 
        },
        unsafe {
            ACCOUNTS.get(&node_second.address).unwrap().balance
        }       
    );
        
        let mut transactons:Vec<Transaction> = Vec::new();
        transactons.push(Transaction::new(node.address, node_second.address, U256([10,0,0,0]), U256([1,0,0,0]), U256([2,0,0,0]), Vec::new(), H256::from([0;32])));
        transactons.push(Transaction::new(node.address, node_second.address, U256([20,0,0,0]), U256([2,0,0,0]), U256([2,0,0,0]), Vec::new(), H256::from([0;32])));

        // 加入发起节点的交易池
        let success = node.send_a_new_transaction(&transactons[0]);
        let success_two = node.send_a_new_transaction(&transactons[1]);

        eprintln!("add to transaction pool : {}",success_two);
        eprintln!("node transaction pool has the tx number : {}",node.transaction_pool.transaction_count);
        

        //开始执行交易，形成pre-block
        //首先来一个miner
        let miner = Miner::new(None);
        eprintln!("Introducing a Miner address {}",miner.address);
        let pre_block = miner.build_block(&mut node, 2);
        assert_eq!(pre_block.miner,miner.address);

        
    }



}