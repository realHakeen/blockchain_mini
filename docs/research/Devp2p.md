# DevP2P

## DevP2P的子协议
1. Wire Protocol
2. les(light ethereum subprotocol)  
3. Snap  Protocol
4. Witness Protocol  
5. Whisper  

## Wire Protocol

### Basic Operation
一旦建立链接，必须要发送Status Message，以太坊的会话就相当于活跃了，然后其他的Mesaage就能够发送。
在会话中，有三个事务：
- 链同步
- 区块传播
- 交易交换
这些任务使用不相交集的协议，客户端通常并行执行他们在不同的对等点连接之间。
客户端的实现应该限制协议的消息大小。需要重点强调的RLPx传输协议限制消息大小在16.7MiB。对于以太坊协议的实际限制会更小，一般是10MiB。如果接受到的Message比限制的大，对等点需要断开链接。

除了接受到的消息需要进行限制，客户端也应该为requests和responses添加软限制。推荐的客户端自行限制的大小和消息的类型有关。对于Request和Responses的限制需要确保是并行处理的，例如，区块同步和交易同步需要在同一个节点链接流畅运行。

### 链同步
参与以太坊协议的节点都需要知道整条链的所有区块数据。链数据可以通过从其他对等点处下载获取。

upon connection，节点间发送他们的Status Message，包含了总难度和他们所知区块的hash值。

客户端首先使用GetBlockMessage消息获取区块头，然后使用GetlockBodies获取区块体，接收到的区块会使用EVM进行执行，再重新创建状态跟和收据。

注意，区块头下载，区块体下载和区块执行都是可以并发执行的。

### 状态同步 也称为“快速同步”
与直接的链同步不同，直接进行状态同步可以实现不允许交易就能同步。协议的版本 eth/63 和 eth/66 均允许同步状态树。自从eth/67以后，状态树是无法再获取了，状态下载由辅助的snap protocol进行。

状态同步通常同过下载区块头验证有效性来运作。区块体会在链同步部分进行下载，但是并不会执行交易，只会验证数据的有效性。客户端随意选择一个接近轴点区块（pivot block）的区块然后下载区块的状态。

### 区块传播
注意：在POW转换到POS之后，区块传播不再有eth协议进行处理。merge 后区块传播由一个新的协议处理，叫做 portal network。portal network 是一个去中心化的网络层，它允许轻量级客户端（如移动设备或浏览器插件）访问以太坊的数据和服务，而不需要下载完整的区块链数据。portal network 由多个子网络组成，每个子网络负责一种特定的数据类型，如状态、历史、交易等。portal network 的目标是提高以太坊的可扩展性和可用性，同时保持去中心化和安全性4。

为什么需要Portal Network？
轻客户端目前无法通过 DevP2P 或 libP2p 请求特定的链数据，因为这些协议仅旨在实现链同步以及区块和交易的八卦。轻客户端不想下载此信息，因为这会阻止它们成为“轻”客户端。Portal Network重新思考整个设计，专门为轻便性而构建，不受现有以太坊客户端的设计限制。  

目标是让轻量级 Portal 客户端的去中心化网络能够：  
- 追踪链头  
- 同步最近和历史链数据  
- 检索状态数据  
- 广播交易  
- 使用EVM执行交易  

portal network 和 stateless ethereum 是两种互补而非替代的技术。portal network 可以让轻量级客户端更容易地访问以太坊上的各种数据和服务，而 stateless ethereum 可以让全节点更容易地执行和验证以太坊上的各种操作。

### 交易交换
所有的节点都必须交换正在pending的交易以便于relay them to miners。客户端对于该实现在Transaction pool中，不同类型的客户端对于交易如何进行包含是不同的。

当一个新的peer connection建立，tx pool在peer 双方都应该进行同步。应该调用NewPooledTransationHashes Msg，一旦收到这个消息客户端就会filter收到的消息，收集在sub pool中没有的tx hash，然后调用GetPooledTransations msg。

当client tx pool中有新的交易出现，客户端应该propagate给网络。

### transaction加密和有效性
交易




# 参考资料
[Ethereum Wire Protocol](https://github.com/ethereum/devp2p/blob/master/caps/eth.md)  
