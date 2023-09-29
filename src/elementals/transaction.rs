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
///
///
///
///
///

/// 首先发现的第一个问题就是设计Address类型。
/// 因此我们构建了Adress的rs文件和research。跳转到（docs/research/ECDSA.md）,为了MVP原则，实现structureDesign.md的内容
/// 完成transaction的函数

pub struct transaction {
    from: Address,
    to:Address,
    amount:U256,
    base_gas:U256,
    priority_fee:U256,
    data:Vec<u8>,
    signature:H256,
}

impl transaction{
    
}



