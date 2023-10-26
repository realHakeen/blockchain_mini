use serde::de::Error;

use crate::elementals::transaction::{self, Transaction};



use crate::Account::account::{ACCOUNTS, self};
pub struct Executor{
    //Excutor需要的状态
}


impl Executor {
    pub fn new(){
        
    }

    // 执行交易，然后把交易的状态进行转换
    pub fn execution_transaction(_transaction:&Transaction)->bool{
        //执行交易的流程是
        //1. 首先验证from的balance，然后验证是否大于amount+gas
        //2. 验证这笔交易的签名是否是有效(暂时不验证)
        //3. 然后将对应用户的balance - amount - gas
        //4. to用户 + amount
        //5. 返回true
        if unsafe { ACCOUNTS.get(&_transaction.from).unwrap().balance > _transaction.base_gas+ _transaction.priority_fee } {
            unsafe{
                ACCOUNTS.get(&_transaction.from).unwrap().balance - _transaction.base_gas - _transaction.priority_fee - _transaction.amount;
                ACCOUNTS.get(&_transaction.to).unwrap().balance + _transaction.amount;
            }
            true
        }else {
            false
        }
    }





    
}