use std::{collections::HashMap, hash::Hash, vec};
use crate::elementals::address::Address;
use primitive_types::U256;
use once_cell::sync::Lazy;


// 负责维护account内对应的代币数量，以太坊中分为EOA和合约账户，我们这里为了MVP，因此仅仅支持EOA。
// 规定balance只有整数，没有小数点
pub struct Account{
    // pub nonce:U256, 暂时不用，这个主要用于防止重放攻击，目前我们不实现
    pub balance:U256,
}

// accounts是一个全局状态，里面维护的是 地址->account的键值对，
// 在Reth中使用的是MDBX作为本地数据库，然后通过p2p协议进行数据传播，而我们这个应用暂时不加入数据库的实现
pub static mut ACCOUNTS:Lazy<HashMap<Address,Account>> = Lazy::new(||{
    let mut map:HashMap<Address,Account> = HashMap::new();
    map
});

impl Account {
    pub fn new()->Self{
        Self { balance: U256([0;4]) }
    }
}