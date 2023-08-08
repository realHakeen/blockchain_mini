# repository structure
core/ : libp2p是所有的crates的依赖项，比如Trasnport和Streammuxer。
transports/ : transport protocol的具体实现（重要）
muxers/ : StreamMuxer的实现，比如(sub)stream multiplexing prtocols
swarm/ : 是libp2p-swarm的Rust实现，主要提供了NetworkBehaviour & ConnectionHandler接口
protocls/ : 基于libp2p-swarm实现的应用协议
misc/ : Utility libraries
libp2p/examples/ : exmaples of 内置的应用协议

# What is Publish/Subscribe
在网络中能够发送Full-message和Metadata-only两种。发送Full-message的称为peers，发送Metadata-only的称为topics。

<image src = "/docs/images/subscribed_peers.png"></image>

## Full-Message
一般情况下Peers只需要连接2-4个对等点即可。  

<image src = "/docs/images/full_message_network.png"></image>  
Peering degree也被称为network degree or D，会影响网络的性能。在libp2p中，默认的D = 6;

## Metadata-only
topics不会传输完整的消息，但是该节点的作用是为我们的网络提供更好的稳健型。
一个节点不能全是Metadata-only节点，要不然就会要求周围的节点升级成Full-message。如果太多Full-message，那么也会要求节点降级为Metadata-only。
libp2p中，每1s（heartbeat）检查一次，并且graft和prune都发生在这1s内。
实际上，Full-message也可以发送Metadata-only数据，属于看人下菜了。

## connected to topic
首先需要subscribe topic，以发送metadata。之后会选择一些节点作为peer，发送Full-message。
**整个设计还是挺巧妙的**  
- 这个topic其实相当于是发散网络的核心，并且作为发散网络的核心，也是网络冗余的保证，那么必然需要metadata-only数据来传输，不能full-message。
- 而且其实每个peers都是一样的，都保留了传输full-message的能力，一旦该节点无full-message的情况下，但是被其他节点选中为full-message，那么就可以从自身选中的peers获取full-message来传输给制定peers。  
- 并且连接是双向的，双方都可以进行prune（裁剪，意味着只需要metadata）和graft（嫁接，意味着full-message）。

当一个peers产生了Message，相当于我们的以太坊节点发起了一笔交易，那么这个peers要主动send Message给peers，wisely，当peers接收到消息后，将自动将message发送给自己连接的peers。在gossipsub specification中，peers也被称为routers。并且peers会维护近期收到的message，以防止重复发送message。

## Gossip
值得注意的是，我们的最底层传输协议，实际上并不负责验证Message的真假，这一工作应该交给更上一级的来做。

