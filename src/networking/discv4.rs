
use crate::elementals::node::Node;
use libp2p::development_transport;
use libp2p::kad::{Kademlia,KademliaConfig,record::store::MemoryStore};
use libp2p::{PeerId,identity,ping,StreamProtocol};
use libp2p::swarm::{SwarmBuilder};
use std::error::Error;
use libp2p::swarm::derive_prelude::NetworkBehaviour;




impl Node {
    //是先是寻找其他的对等点peer，根据Discv4协议的specs设计
    ///我们主要使用Kademlia + ethereum node discover protocol
    pub async fn creat_kademlia_network()->Result<(),Box<dyn Error>> {
        //创建一个本地节点
        let local_key = identity::Keypair::generate_ed25519();
        let public_key = local_key.public();
        let local_peer_id = PeerId::from_public_key(&public_key);
        //使用基于内存的存储
        let mut store = MemoryStore::new(local_peer_id);
        //创建一个具有默认配置的kademlia网络实现,它可以执行各种查询操作，例如获取最近的节点、获取记录、发布记录等
        let kademlia = Kademlia::new(local_peer_id,store);
        //创建网络管理器
        //构建transport和swarm，通过swarm将transport和behaviour链接起来

        #[derive(NetworkBehaviour)]
        struct Behaviour{
            kademlia:Kademlia<MemoryStore>
            
        }
        
        let behaviour: Behaviour = Behaviour{
            kademlia,
        };
        let transport = development_transport(local_key).await?;


        //创建并且运行swarm
        let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();
        //swarm是一个管理本地节点和其它节点之间连接和事件的类型
        Ok(())
    }

    pub fn scan_network(){
        
    }
}
