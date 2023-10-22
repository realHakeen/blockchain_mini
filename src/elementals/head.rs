use std::{time::SystemTime, os::unix};

use primitive_types::*;
#[derive(Debug,Clone, Copy)]
pub struct Header{
    parent_block:H256,
    timestamp:SystemTime,
    gas_used:U256,
}


impl Header {
    pub fn new(_parent_block:H256,_timestamp:SystemTime,_gas_used:U256) -> Self{
        Header { parent_block: _parent_block, timestamp: _timestamp, gas_used: _gas_used }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}, {:?} , {})",self.parent_block,self.timestamp,self.gas_used)
    }
}

mod tests{
    use std::time::SystemTime;
    use crate::elementals::head::{Header};
    use primitive_types::{self, H256, U256};
    # [test]
    fn test_header(){
        let block_hash = H256([0;32]);
        let header = Header::new(block_hash, SystemTime::now(), U256([10,0,0,0]));
        eprint!("{}",header.to_string());
    }

}