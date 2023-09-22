use crate::elementals::node::Node;




pub async fn test_node_create(){
    let node = Node::new();
    println!("public_key of the node : {:?}",node.public_key);
    println!("address of the node : {:?}",node.address);
}
