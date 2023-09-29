use std::{time::SystemTime, os::unix};

use primitive_types::*;

pub struct Header{
    parent_block:H256,
    timestamp:U256,
    gasUsed:U256,
}