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