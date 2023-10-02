use std::str::Bytes;

use crate::elementals::address::Address;
use primitive_types::*;
///
/// 根据README.md介绍，交易的字段包括
/// 1. from：该交易的发起人
/// 2. to：该交易的接受人
/// 3. amount：转账的数量
/// 4. fee 转账需要的费用
/// 5. data：转账携带的数据
/// 6. signature:签名
///


/// 首先发现的第一个问题就是设计Address类型。
/// 因此我们构建了Adress的rs文件和research。跳转到（docs/research/ECDSA.md）,为了MVP原则，实现structureDesign.md的内容
/// transaction.rs内维护transaction pool池子

pub struct Transaction {
    pub from: Address,
    pub to:Address,
    pub amount:U256,
    pub base_gas:U256,
    pub priority_fee:U256,
    pub data:Vec<u8>,
    pub signature:H256,
}


impl Transaction{
    
}



