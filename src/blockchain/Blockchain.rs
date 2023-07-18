use super::block::Block;
use chrono::prelude::*;

type Blocks = Vec<Block>;
pub struct Blockchain {
    //genesis_block
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {}
