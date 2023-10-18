use crate::Account::account::ACCOUNTS;
use crate::Validation::transactionValidation;
use crate::elementals::node::Node;
use crate::elementals::transaction::{Transaction, self};
use crate::TransactionPool::transactionpool::TransactionPool;


/// miner负责存入交易，调用交易的校验，目前仅仅校验交易中，
/// 转移数量的合法性，validation调用aacount查看状态
/// miner负责从txpool中挑选gas price最高的transaction，形成block
/// miner角色维护的数据暂时无
# [derive(Debug,Clone)]
pub struct Miner{
    
}



impl Miner {
    pub fn new(){
        todo!()
    }
    // pick transactions 是从transactionpool里面找到一些transactions,n的数量是miner自己设置的，node是不管的
    // 我们一般认为当n = 10 或者slot在12s时，会pick transactions 发送给evm执行交易（目前evm只支持转账），并且打包成pre-block
    // 
    fn pick_transactions(_node:&mut Node,n:usize)->Vec<Transaction>{
        _node.transaction_pool.get_transaction_in_fcfs(n)
    }
    // evm负责把我们pick的transactions
    fn build_block(&self){
        todo!()
    }
    
}