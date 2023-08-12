# Reth
Reth是由Paradigm研发的以太坊执行层+共识层客户端，基于Rust语言。目前还在开发阶段，本章节，我们主要是学习一下Reth的Layout，这也对我们研发我们自己的客户端有一定参考性。  
Reth主要分为以下几个components：
- Storage：负责交易数据的存储以及全局状态  
- newtworking：负责执行层的消息传递  
- Consensus：负责共识层的消息传递，比如Libp2p和Gossipsub的实现  
- Execution：负责执行层的EVM虚拟机  
因此，我们的项目也根据该方式来做。必要时，我们将使用Reth的crates。因此我们的src文件也将使用这种文件结构。




# 参考资料
[RETH](https://github.com/paradigmxyz/reth/blob/main/docs/repo/layout.md)  