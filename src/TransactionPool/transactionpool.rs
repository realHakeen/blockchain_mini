
use std::collections::HashMap;
use crate::elementals::address::{Address, self};
use crate::elementals::transaction::{Transaction, self};
use crate::Account::account::{Account,ACCOUNTS};
/// 这个mod负责处理交易池子的同步，将验证（Validation模块处理）后的交易放入池子中
/// transactionpool流程：who do what
/// Node发起一笔交易，调用txpool的api，放入txpool，txpool负责验证该交易，如果验证成功则放入返回true，否则返回false
/// txpool里面都是tx，还有一些状态，如tx的个数，为了MVP，没有数据结构组织，暂定为线性
/// transactionpool需要提供方便的gas price排序，选择前N个
/// 
/// 
#[derive(Debug,Clone)]
pub struct TransactionPool{
    transactionpool:Vec<Transaction>,
    transaction_count:u128,
}

// miner负责与transactionpool进行交互，其为主体

trait TransactionPoolInterface{
    fn get_transaction_pool_count(transactionPool:TransactionPool) -> u128;
    fn get_transaction_in_order();
}

impl TransactionPool {
    pub fn new() -> Self{
        TransactionPool { transactionpool: vec![], transaction_count: 0 }
    }
    fn validate_transaction(_address:Address,_transaction:Transaction) -> bool{
        /// 查看该address下的account是否有被初始化
        /// 首先查找该地址下的Account对应的balance，确定balance是否大于amount+gas(暂时不要gas)，如果大于则通过validation
        /// 当然还需要验证该transaction的signature是否满足，但是我们暂时不实现
        assert_eq!(_address,_transaction.from);
        if unsafe { ACCOUNTS.contains_key(&_address) } {
            let balance = unsafe { ACCOUNTS.get(&_address).unwrap().balance };
            if balance >  _transaction.amount{
                true
            }else {
                false
            }
        }else {
            //在Accounts中创建，然后返回false
            let account = Account::new();
            unsafe{
                ACCOUNTS.insert(_address, account);
            }
            false
        }
    }
    pub fn validate_signature(_address:Address,_transaction:Transaction){
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

mod tests{
    use primitive_types::{U256, H256};
    use secp256k1::{rand,PublicKey, KeyPair, Secp256k1};

    use crate::Account::account::{Account, ACCOUNTS};
    use crate::TransactionPool::transactionpool::TransactionPool;
    use crate::elementals::node::Node;
    use crate::elementals::transaction::Transaction;
    use crate::elementals::address::{self, Address};
    # [test]
    // 构建一个transaction，以及transactionpool，看函数是否执行到位
    fn test_tx_into_txpool(){
        let secp = Secp256k1::new();
        let keypair = KeyPair::new(&secp, &mut rand::thread_rng());
        let mynode = Node::new(Some(keypair.secret_key()));
        let my_address = Address::get_address_from(keypair.public_key());
        unsafe { ACCOUNTS.insert(my_address, Account { balance: U256::from(100) }) };
        let one_transaction = Transaction::new(my_address,
            my_address,
            U256::from(10),
            U256::from(10),
            U256::from(10),
            vec![],
            H256::from_low_u64_be(0)
        );
        // 初始化完成以后，即可测试
        let test = TransactionPool::validate_transaction(my_address, one_transaction);
        assert_eq!(true,test);
    }
}

