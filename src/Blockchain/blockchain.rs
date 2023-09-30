use std::time::SystemTime;

use crate::elementals::block::{Block};
use crate::elementals::head::Header;
use primitive_types::*;
use crate::elementals::address::Address;

///blockchain由一个链接一个的block组成
pub struct Blockchain{
    block:Vec<Block>,
}

///首先我们要创建一个blockchain，创世区块都是一样的
impl Blockchain {
    /// new一个新的blockchain，第一个block一定是genesisblock，以太坊也是一开始就配置好的
    pub fn new() -> Self{

        let mut genensis_block = Block::new(
            Header::new(H256([0;32]), SystemTime::UNIX_EPOCH, U256([0;4])),
            U256([0;4]),
            H256([0;32]),
            Address(H160([0;20])),
        );
        Blockchain { block: vec![genensis_block] }
    }
    /// 添加被共识确认后的block，因为实际上可能同时存在N个block
    pub fn add_saledBlock(mut self,_saled_block:Block){
        self.block.push(_saled_block);
    }
}
