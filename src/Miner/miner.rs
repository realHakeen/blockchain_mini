use crate::Account::account::ACCOUNTS;
use crate::Validation::transactionValidation;
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
    pub fn send_a_new_transaction(&self,_transaction:Transaction,_transaction_pool:TransactionPool){
        
    }
    fn pick_transactions(){
        todo!()
    }
    fn build_block(){
        todo!()
    }
    
}