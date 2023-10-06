use std::time::SystemTime;

use crate::elementals::{peerid::PeerID, node::Node};




# [derive(Debug)]
pub struct NodeDiscvService{
    pub node: PeerID,
    pub node_list:Option<Vec<PeerID>>,
}
// NodeDiscvService负责bind一个peerid，然后为其维护一个Node list，这些node list只是负责通信。
impl NodeDiscvService {
    pub fn bind(_node_peerid:PeerID)->Self{
        Self { node: _node_peerid, node_list: None }
    }
    /// find_other_node的逻辑主要是寻找其它的PeerId，维护PeerId列表
    /// 使用的查找算法因为MVP原则，暂时不实现
    pub fn find_other_node(&self){
        todo!()
    }
}

// 将找到的节点，获取节点的信息：主要是获得其它节点的blockchain以及transactionpool，并且会在给定的slot时间自动调用
// 这里使用异步技术实现
impl NodeDiscvService {
    pub fn connection(&self){
        todo!()
    }
    pub fn get_message_from_other_nodes(&self){
        todo!()
    }
    pub async fn get_message_from_other_nodes_per_slot(&self,slot:SystemTime){
        todo!()
    } 
}
