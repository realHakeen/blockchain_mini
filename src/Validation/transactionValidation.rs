
use crate::elementals::transaction::{self, Transaction};
use secp256k1::hashes::Hash;
use secp256k1::{SecretKey, Message};
use crate::elementals::address::Address;
use crate::Account::account::{Account,ACCOUNTS};

/// Validator需要验证的交易方面包括
/// 1. 验证交易的发起地址是否为from地址（address, transaction）-> Bool
/// 2. 验证交易的数量是否小于全局状态ACCOUNTS内记录的balance数量
/// 3. 验证该笔交易的签名

pub struct Validator{

}

impl Validator {
    pub fn sign_transaction(_secrate_key:SecretKey,_transaction:Transaction){
        let secrate_key = _secrate_key;
    }

    
    pub fn validate_transaction(_address:Address,_transaction:Transaction) -> bool{
        /// 查看该address下的account是否有被初始化
        /// 首先查找该地址下的Account对应的balance，确定balance是否大于amount+gas，如果大于则通过validation
        /// 当然还需要验证该transaction的signature是否满足，但是我们暂时不实现
        if unsafe { ACCOUNTS.contains_key(&_address) } {
            let balance = unsafe { ACCOUNTS.get(&_address).unwrap().balance };
            if balance >  _transaction.amount+ _transaction.base_gas+_transaction.priority_fee{
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

    fn signature_validation(_address:Address,_transaction:Transaction)->bool{
        todo!()
    }

    
}







mod tests{
    use primitive_types::{U256, H256};
    use secp256k1::{rand,PublicKey, KeyPair, Secp256k1};

    use crate::Account::account::{Account, ACCOUNTS};
    use crate::TransactionPool::transactionpool::TransactionPool;
    use crate::Validation::transactionValidation::Validator;
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
        let test = Validator::validate_transaction(my_address, one_transaction);
        assert_eq!(true,test);
    }
}