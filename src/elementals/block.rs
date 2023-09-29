use primitive_types::{U256,H256};
use crate::elementals::address::Address;
use crate::elementals::head::Header;

// block的设计specs，可以在DevP2P.md文件中找到，但是为了MVP原则，我们现在所有的设计均使用structureDesign.md中的设计

pub struct Block{
    header:Header,
    block_number:U256,
    transaction_root:H256,
    miner:Address,
}



/// Block 需要完成的func如下所示：
/// 
/// 
impl Block {
    
}
