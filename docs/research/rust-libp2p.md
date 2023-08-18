# repository structure
- core/ : libp2p是所有的crates的依赖项，比如Trasnport和Streammuxer。
- transports/ : transport protocol的具体实现（重要）
- muxers/ : StreamMuxer的实现，比如(sub)stream multiplexing prtocols
- swarm/ : 是libp2p-swarm的Rust实现，主要提供了NetworkBehaviour & ConnectionHandler接口
- protocls/ : 基于libp2p-swarm实现的应用协议
- misc/ : Utility libraries
- libp2p/examples/ : exmaples of 内置的应用协议

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
- 这个topic其实相当于是发散网络的核心，并且作为**发散网络的核心**，也是网络冗余的保证，那么必然需要metadata-only数据来传输，不能full-message。topic是所有metadata-only消息传递能力节点组成的group。
- 而且其实每个peers都是一样的，都保留了传输full-message的能力，一旦该节点无full-message的情况下，但是被其他节点选中为full-message，那么就可以从自身选中的peers获取full-message来传输给制定peers。  
- 并且连接是双向的，双方都可以进行prune（裁剪，意味着只需要metadata）和graft（嫁接，意味着full-message）。

当一个peers产生了Message，相当于我们的以太坊节点发起了一笔交易，那么这个peers要主动send Message给peers，wisely，当peers接收到消息后，将自动将message发送给自己连接的peers。在gossipsub specification中，peers也被称为routers。并且peers会维护近期收到的message，以防止重复发送message。

## Gossip
值得注意的是，我们的最底层传输协议，实际上并不负责验证Message的真假，这一工作应该交给更上一级的来做。
per second，每个peer都会发送给6个topic节点list。Gossip会提示节点是否遗漏message，如果多次遗漏，那么就会建立新的peers链接。

## Fan-out
Peers可以发送message给未subsribe的topics。首先选择6个订阅了的topic节点，然后将这6个节点记为fan-out节点。
fan-out peering是单向的，如果想传递message，可以先传递给fan-out，然后fan-out负责在topic内广播。
很多时候，fan-out节点只是临时的措施，fan-out节点会最终变成full-message节点，直到自己被纳入整个topic。

<image src = "/docs/images/fanout_grafting_preference.png"></image>
两分钟内未发送任何新的message，会自动prune成meta-data peering。

## floodpsub
gossipsub是使用gossip算法实现的pubsub specifications，而floodsub是其中一种实现。
我们可以先了解floodsub。
floodsub路由策略具有以下非常理想的属性：
- 试试起来很简单  
- 它最大程度地减少了延迟  
- 它非常稳定，需要管理的逻辑和状态非常少  

# gossipsub八卦网络路由
gossipsub想要解决floodsub的一些问题，主要手段是将每个peer的degree施加上限，并且控制增强因子。
gossipsub颠覆了常规的实现方式，不是使用topic，而是通过一个subset集合（称为mesh），send message给subset而不是整个topic，subset是在topic内进行选择的。

当进入topic，peer会发送给subset Graft控制协议，而当离开topic会给subset发送prune。当subset内的peer收到来自其他peer的graft控制消息，那么它会将来源peer也加入到自己的mesh。所以如果peer A是peer B的mesh，那么peer B也是peer A的mesh。

我们需要辨析的点在于，topic是用于subscripe和unsubscribe的，只是用来消息的同步的，但是mesh是用来传递full-message的，那么在topic内剩下的都传递metadata-only。



以下是以太坊交易的全流程图示：  

<image src = "/docs/images/lifecycle.png"></image>  
我们能够看到，以太坊的共识层与执行层之间消息传递使用的协议是不一样的。接下来，我们看看以太坊的执行层消息传递协议。[执行层消息传递协议](/docs/research/Devp2p.md)    




# 参考资料
[gossipsub-v1.0](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/gossipsub-v1.0.md#motivations-and-prior-work)  
