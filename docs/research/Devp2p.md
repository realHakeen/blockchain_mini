# DevP2P
根据以太坊的官网简述，我们的执行层与共识层使用的消息传递协议是不一样的。
在执行层中，我们使用Devp2p，分为两方面：
1. 发现协议堆栈：建立在UDP之上，允许新节点找到要链接的对等点，发现协议称为Discv4，现在正在努力迁移到Discv5。paradigm的RETH也为其写了一个[rust版本的Discv4](https://github.com/paradigmxyz/reth/tree/427a8395f9d89aaab8ff563d6041af34cddbf425/crates/net/discv4)  
2. DevP2P：位于TCP之上，使节点能够交换信息。DevP2P 本身就是以太坊为了建立和维护点对点网络而实现的一整套协议。新节点进入网络后，它们的交互由 DevP2P 中的协议控制堆栈。这些协议均位于 TCP 之上，包括 RLPx 传输协议、有线协议和多个子协议。  

## DevP2P的子协议
1. Wire Protocol
2. les(light ethereum subprotocol)  
3. Snap  Protocol
4. Witness Protocol  
5. Whisper  

## Wire Protocol
