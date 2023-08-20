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

### transaction格式和有效性
peers交换的交易对象有两种格式。
```
tx = {legacy-tx,typed-tx}
```
legacy trasation是一个RLP list。
```
legacy-tx = [
    nonce:P,
    gas-price:P,
    gas-limit:P,
    recipient:{B_0,B_20},
    vale:P,
    data:P,
    V:P,
    R:P,
    S:P,
]
```
EIP-2718类型的交易被编码成RLP byte arrays，第一个字节是交易类型tx-type，剩下的内容都是透明的数据。
typed-tx = tx-type || tx-data
交易必须被验证，当接收时。有效性是依赖于以太坊的链状态的。有效性不是交易是否被EVM成功执行，而是指该交易是否被接受并且短暂的存储在local pool中，并且与其它peers exchange。

**交易如何验证的呢？**
- 被定义过的交易可以视为有效，甚至在他们在被包含在block中之前，只要知道tx-type都可以视为有效。
- 签名必须有效。对于typed tx，签名都是由对应定义type-tx的EIP进行处理。对于leagcy-tx，目前有效使用的方案都是‘Homestead’方案和EIP-155方案。
- gas-limit必须cover intrinsic gas。
- sender account必须有足够的gas-limit * gas-price + value 的balance
- nonce必须等于或者大雨sender的目前nonce
- 对于不同的客户端实现，每个实现对于nonce如何进行处理都不一样。

### Block encoding和有效性
EVM blocks如下进行编码：
```
block = [header, transactions, ommers]
transactions = [tx₁, tx₂, ...]
ommers = [header₁, header₂, ...]
withdrawals = [withdrawal₁, withdrawal₂, ...]
header = [
    parent-hash: B_32,
    ommers-hash: B_32,（ommers是一种有效但是并没有被包含到主链到区块）
    coinbase: B_20,
    state-root: B_32,
    txs-root: B_32,
    receipts-root: B_32,
    bloom: B_256,
    difficulty: P,
    number: P,
    gas-limit: P,
    gas-used: P,
    time: P,
    extradata: B,
    mix-digest: B_32,
    block-nonce: B_8,
    basefee-per-gas: P,
    withdrawals-root: B_32,
]
```
对于single block header，只有POW的有效性能够被验证。当header用于拓展client的local chain有以下几个规则：
- parent-hash必须match
- 当拓展local-stored chain，客户端必须验证difficulty、gas-limit、time等字段
- gas-used必须小于等于gas-limit
- base-fee per gas必须加入header中，在london升级之后，该规则在EIP-1559中定义
- POS blocks，ommers-hash必须是empty keccak256 hash，因为没有ommer headers会存在，因为POS的独特机制
- withdrawals-root必须包含在shanghai升级之后

对于完整的区块，我们会区分区块的EVM状态转换的有效性和弱数据有效性。我们需要da以进行区块传播在状态同步过程中。

为了验证block的data validity，我们使用以下规则：
- 区块头必须有效
- 需要验证tx-type是否允许包含在区块中，and tx gas的有效性也需要考虑在内
- gas limit总和不能超过block gas limit
- transaction必须通过txs-root进行验证
- block body的withdrawls字段通过withdrawals-root验证

### Receipt编码和有效性
Receipts是EVM状态转换后的结果。与transation一样，receipts也有两种编码方式：
```
receipt = {legacy-receipt,typed-receipt}
```

```
legacy-receipt = [
    post-state-or-status: {B_32, {0, 1}},
    cumulative-gas: P,
    bloom: B_256,
    logs: [log₁, log₂, ...]
]
log = [
    contract-address: B_20,
    topics: [topic₁: B, topic₂: B, ...],
    data: B
]
```
eip-2718 typed-receipt以RLP编码的形式进行编码，必须要加上tx-type。
```
typed-receipt = tx-type || receipt-data
```
在Ethereum Wire Protocol中，receipts总是以一个receipt list进行传输，每个receipt 对应一笔交易。当一个peer接收到receipts-root后会校验。

### protocol message
在大多数msg中，msg list的第一个元素是request-id。
- NewBlockHashed(0x01)  
- Transactions(0x02)  
- GetBlockHeaders(0x03)  
- BlockHeaders(0x04)  
- GetBlockBodies(0x05)  
- BlockBodies(0x06)  
- NewBlock(0x07)  
- NewPooledTransactionHashes(0x08)  
- GetPooledTransactions(0x09)  
- PooledTransactions(0x0a)  
- GetReceipts(0x0f)
- Receipts(0x10):Response for the GetReceipts  
 
## Others
DevP2P最重要和核心的协议是Wire Protocol，以下的其它协议我们只需要简单介绍，是可选实现的。
### Ethereum Snapshot Protocol
以太坊的 snap protocol 是一种可选的扩展协议，它允许节点之间交换最近状态的快照，从而让节点可以验证账户和存储数据，而无需下载中间的默克尔树节点。

snap protocol 的目的是为了提高以太坊的同步效率和可靠性，特别是在使用快速同步或轻量级客户端的情况下。snap protocol 可以用来获取数据，比如账户余额、代码、存储值、交易收据等。snap protocol 也可以用来获取区块头、区块体和区块哈希等信息。

snap sync 是一种新的同步模式，它使用了一种叫做 snap protocol 的扩展协议，可以让节点之间交换最近状态的快照，从而让节点可以验证账户和存储数据，而无需下载中间的默克尔树节点。snap sync 的优点是它可以大大减少数据的传输量和存储空间，同时提高数据的可靠性和可迭代性。snap sync 的缺点是它需要一个初始的信任点，也就是一个最近的区块哈希，来开始同步，并且它还没有被所有的以太坊客户端实现。 

### Light Ethereum Subprotocol(LES)
light ethereum subprotocol (LES) 是一种用于轻客户端的协议，它只下载最新的区块头，并根据需要获取区块链的其他部分。它提供了完整的功能，可以安全地访问区块链。

### Parity Light Protocol (PIP)
Parity Light 协议是 LES 的变体，由 Parity Tech 为 Parity 以太坊客户端设计和实现。

### Ethereum Witness Protocol (wit)
Ethereum Witness Protocol (wit) 是一种在 RLPx 传输层上运行的协议，它可以让节点之间交换以太坊状态的证明（witness）。这个协议是一个可选的扩展，用于支持或关心以太坊区块的状态证明的节点。目前的版本是 wit/0。

wit 协议的目的是为了帮助节点同步到链的最新状态。未来，它也希望能够帮助无状态客户端（stateless client）的运行。**wit 协议不参与链的维护（区块和交易的传播），而是与 eth 协议并行运行**，而不是独立运行（例如，链的进展是通过 eth 来通知的）。

尽管名字叫做 witness protocol，但是版本 0 并不提供真正的 witness。它只提供了 witness 的元数据，可以用来通过 eth 协议来下载 witness。目前，已知的用例是为了帮助 Beam Syncing 的节点。通过请求 witness 元数据，这些节点可以跟上网络的最新状态，并更快地成为完全同步的节点。

# 参考资料
[Ethereum Wire Protocol](https://github.com/ethereum/devp2p/blob/master/caps/eth.md)  
[Ethereum protocol](https://github.com/ethereum/devp2p/tree/master)  
