use std::str::Bytes;

use primitive_types::{U256,H256};
use crate::elementals::address;

// block的设计specs，可以在DevP2P.md文件中找到

type Address = H256;
type bytes32 = H256;
pub struct Block{
    //夫区块的哈希值
    parent_hash:bytes32,
    //在Casper FFG共识中，POW与POS共识并存，仍然存在叔区块
    ommers_hash:bytes32,
    coinbase:Address,
    state_root:bytes32,
    transation_root:bytes32,
    receipts_root:bytes32,
    logs_bloom:bytes32,
    difficulty:U256,
    number:U256,
    gas_limit:U256,
    gas_used:U256,
    timestamp:U256,
    extra_data:bytes32,
    mix_hash:bytes32,
    nonce:u64,
}


/// Block 需要完成的func如下所示：
/// 
/// 
impl Block {
    
}
