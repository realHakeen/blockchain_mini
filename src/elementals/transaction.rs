use Address;
///
/// 根据README.md介绍，交易的字段包括
/// 1. from：该交易的发起人
/// 2. to：该交易的接受人
/// 3. amount：转账的数量
/// 4. fee 转账需要的费用
/// 5. data：转账携带的数据
///
///
///
///
///
///
///

/// 首先发现的第一个问题就是设计Address类型。
/// 因此我们构建了Adress的rs文件和research。跳转到（docs/research/ECDSA.md）
/// 完成transaction的函数

pub struct transaction {
    from: Address,
    to: Address,
    amount: usize,
    fee: usize,
    data: String,
}

impl transaction{

}
