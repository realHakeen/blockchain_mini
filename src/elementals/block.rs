use std::fmt::{write, Display};
use std::str::FromStr;

use primitive_types::{U256,H256};
use crate::elementals::address::Address;
use crate::elementals::head::Header;

// block的设计specs，可以在DevP2P.md文件中找到，但是为了MVP原则，我们现在所有的设计均使用structureDesign.md中的设计
#[derive(Debug,Clone, Copy)]
pub struct Block{
    header:Header,
    pub block_number:U256,
    transaction_root:H256,
    miner:Address,
}





/// Block 需要完成的func如下所示：
/// 
/// 
impl Block {
    pub fn new(
        _header:Header,
        _block_number:U256,
        _transaction_root:H256,
        _miner:Address,
    ) ->Self {
        Block { header: _header, block_number: _block_number, transaction_root: _transaction_root, miner: _miner }
    }

    pub fn as_hash(&self)->H256{
        // 首先把Block转换成str类型
        todo!()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}, {}, {}, {})",self.header,self.block_number,self.transaction_root,self.miner)
    }
}






