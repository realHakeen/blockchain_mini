
use std::collections::HashMap;
use crate::elementals::address::{Address, self};
use crate::elementals::transaction::{Transaction, self};
use crate::Account::account::{Account,ACCOUNTS};
use crate::Validation::transactionValidation::Validator;
/// 这个mod负责处理交易池子的同步，将验证（Validation模块处理）后的交易放入池子中
/// transactionpool流程：who do what
/// Node发起一笔交易，调用txpool的api，放入txpool，txpool负责验证该交易，如果验证成功则放入返回true，否则返回false
/// txpool里面都是tx，还有一些状态，如tx的个数，为了MVP，没有数据结构组织，暂定为线性
/// transactionpool需要提供方便的gas price排序，选择前N个，因此需要算法来实现gas price排序
/// 
/// 
#[derive(Debug,Clone)]
pub struct TransactionPool{
    pub transactionpool:Vec<Transaction>,
    pub transaction_count:u128,
}

// miner负责与transactionpool进行交互，其为主体
// transactionpool负责调用Validation模块进行传入的交易验证
// 验证成功后，插入transactionpool，记住此时仍然是未执行的状态，还有一个问题需要处理就是如果我们同时发送两笔交易
// 那么此时验证一样会通过validation，这时，可以重现重放攻击(signature验证解决)或者双花(nounce验证排序解决，在最后更改状态后一样要验证balance的)
// 暂时不予实现


impl TransactionPool {
    pub fn new() -> Self{
        TransactionPool { transactionpool: vec![], transaction_count: 0 }
    }
    
    /// 一旦完成transaction的验证，那么就将该transaction放入txpool中
    pub fn add_tx_to_transaction_pool(&mut self,_address:Address,_transaction:Transaction)->bool{
        let success = Validator::validate_transaction(_address, _transaction.clone());
        if success{
            // 初步验证交易成功，则把交易放到transactionpool中，我们放入txpool是fcfs原则，但是具体打包交易挑选什么block则由miner来决定
            self.transactionpool.push(_transaction.clone());
            self.transaction_count += 1;
            true
        }else {
            false            
        }
    }
}

impl TransactionPool {
    pub fn get_transaction_in_price_order(&self,n:usize){
        // 进行排序，按照gas fee排序，选择前n个transaction
        let mut ordered_transactionpool:Vec<Transaction> = Vec::new();
        //暂时我们不实现排序算法
        ordered_transactionpool = self.transactionpool.clone();
       
        //然后pop
        todo!()
    }
    pub fn get_transaction_in_fcfs(&mut self,n:usize)->Vec<Transaction>{
        // 首先把需要获取的transactions获得之后，要把这前n个transactions pop出去
        let return_transactions:Vec<Transaction> = self.transactionpool.iter().take(n).cloned().collect();
        // 把transactionpool pop出去
        self.transactionpool.split_off(n);
        return_transactions
    }
}



mod tests{

    # [test]
    fn test_tx_into_txpool(){
        
    }
}


