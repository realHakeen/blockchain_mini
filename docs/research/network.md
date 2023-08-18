
# Network

Networking层就是我们所说的Execution Layer，其中包括两个协议
1. 节点发现，使用的是[Discv4/5](/docs/research/Discv4.md)  
2. 节点通信，使用的是[Devp2p协议](/docs/research/Devp2p.md) 

根据以太坊的官网简述，我们的执行层与共识层使用的消息传递协议是不一样的。
在执行层中，我们使用的协议分为两方面：
1. 发现协议堆栈：建立在UDP之上，允许新节点找到要链接的对等点，发现协议称为Discv4，现在正在努力迁移到Discv5。paradigm的RETH也为其写了一个[rust版本的Discv4](https://github.com/paradigmxyz/reth/tree/427a8395f9d89aaab8ff563d6041af34cddbf425/crates/net/discv4)  
2. DevP2P：位于TCP之上，使节点能够交换信息。DevP2P 本身就是以太坊为了建立和维护点对点网络而实现的一整套协议。新节点进入网络后，它们的交互由 DevP2P 中的协议控制堆栈。这些协议均位于 TCP 之上，包括 RLPx 传输协议、有线协议和多个子协议。  

## 节点在网络中的使用
运行节点在node 层面，需要以下操作：
1. 初始化数据库
2. 初始化共识API
3. 将创世块写入数据库
4. 初始化网络
5. 实例化客户端以从网络获取数据
6. 通过向管道添加阶段来配置管道
7. 运行管道

# 参考资料
[Reth of network docs](https://github.com/paradigmxyz/reth/blob/main/docs/crates/network.md)  
