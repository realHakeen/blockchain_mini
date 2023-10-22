
use crate::VM::transactionExecute::Executor;
use crate::Account::account::ACCOUNTS;
use crate::Validation::transactionValidation;
use crate::elementals::head::Header;
use crate::elementals::node::Node;
use crate::elementals::transaction::{Transaction, self};
use crate::elementals::block::Block;


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
    // 方法
    fn pick_transactions(_node:&mut Node,n:usize)->Vec<Transaction>{
        _node.transaction_pool.get_transaction_in_fcfs(n)
    }
    // evm负责把我们pick的transactions执行
    fn build_block(_node:&mut Node,n:usize){
        //首先是VM执行交易，然后全部成功的话，就打包成一个block
        let transactions = Miner::pick_transactions(_node, n).into_iter();
        for i in transactions {
            let success = Executor::execution_transaction(i);
            if success == false {
                todo!()
            }else {
                continue;
            }
        }
        // 形成一个pre-block
        
        // 获取parent_block，要从哪里获得？
        

        //然后这个block一旦被consensus网络达成共识，那么即可成为block
        //在以太坊中，所有的交易不会成为

    }
    
}