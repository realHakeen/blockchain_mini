
use crate::elementals::transaction::Transaction;

/// 这个mod负责处理交易池子的同步，将验证（Validation模块处理）后的交易放入池子中
/// transactionpool流程：who do what
/// Node发起一笔交易，调用txpool的api，放入txpool，txpool负责验证该交易，如果验证成功则放入返回true，否则返回false
/// txpool里面都是tx，还有一些状态，如tx的个数，为了MVP，没有数据结构组织，暂定为线性
/// transactionpool需要提供方便的gas price排序，选择前N个
pub struct TransactionPool{
    transactionpool:Vec<Transaction>,
    transaction_count:u128,
}

trait TransactionPoolInterface{
    fn get_transaction_pool_count(transactionPool:TransactionPool) -> u128;
    fn get_transaction_in_order();
}

impl TransactionPool {
    pub fn new() -> Self{
        TransactionPool { transactionpool: vec![], transaction_count: 0 }
    }
    fn validate_transaction(transaction:Transaction) -> bool{
        todo!()
    }
    /// 一旦完成transaction的验证，那么就将该transaction放入txpool中
    pub fn add_transaction_pool(transaction:Transaction){
        todo!()
    }
}

impl TransactionPoolInterface for TransactionPool {
    fn get_transaction_pool_count(transactionPool:TransactionPool) -> u128{
        transactionPool.transaction_count
    }
    fn get_transaction_in_order() {
        todo!()
    }
}



