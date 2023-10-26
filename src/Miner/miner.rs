
use std::borrow::Borrow;
use std::time::SystemTime;

use primitive_types::{U256, H256};

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
    address:Address,
}



impl Miner {
    pub fn new(){
        todo!()
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

    use crate::VM::transactionExecute::Executor;
    use crate::Account::account::ACCOUNTS;
    use crate::Validation::transactionValidation;
    use crate::elementals::head::Header;
    use crate::elementals::node::Node;
    use crate::elementals::transaction::{Transaction, self};
    use crate::elementals::block::Block;
    use crate::elementals::address::Address;

    # [test]
    fn test_build_block(){
        //运行一个节点
        let node = Node::new(None);
        //插入多笔交易
        todo!()

    }



}