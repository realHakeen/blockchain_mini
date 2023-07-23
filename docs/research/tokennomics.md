## Prague的代币经济学
实际上设计代币的经济模型非常复杂，由于整条链的矿工打包和共识达成会对代币经济学的基本面产生实质上的影响，因此我们对其进行简要的设计。这是txpool设计需要着重设计的细节，也就是**整个protocol的代币经济学**所在，我们认为一条公链更应该以priority fee来进行交易，但是这会加剧MEV的出现。以太坊自从**EIP-1559**之后，使用的就是base fee(burn)+ priority fee(for workers)的经济学。但是随着即将到来的Prague+Elatra升级引入的Proto-Danksharding，会让以太坊的代币经济学更加复杂难以预测。

目前，以太坊共识节点和执行节点是合并后的以太坊网络中的两种不同类型的节点。执行节点负责处理交易和智能合约，执行以太坊虚拟机，并保存最新的以太坊状态。共识节点负责实现权益证明共识算法，协调验证者网络，并根据执行节点的数据达成网络一致性。

执行节点和共识节点的收费方式不同。执行节点收取交易手续费，这是由交易的gas消耗量和gas价格决定的。gas消耗量是由合约逻辑决定的，gas价格是由用户自己设定的。gas价格反映了用户愿意为这笔交易支付的费用，也影响了交易被打包的速度。一般用Gwei做单位，$1 eth = 10^9 Gwei$。

共识节点收取质押奖励，这是由验证者在信标链上质押32个eth，并参与区块验证和提案而获得的。质押奖励与质押总量、验证者在线率、验证者数量等因素有关。

### tokenEconimics
以太坊的经济机制一直以来都是First-price auction，现在引入了EIP-1559，将最大区块空间从12.5M，该成了15M-30M，**我们的目标traget block usage应该在50%，也就是15M**。当区块空间每增大12.5%，那么base fee就会增大，具体计算公式如下：
base fee $r_{cur}$
the prev block base fee $r_{pred}$
size of the predecessor block $s_{pred}$

那么整个公式就是：$r_{cur} := r_{pred}* (1+ \frac{1}{8} * \frac{s_{pred}-s_{target}}{s_{target}})$

* 如果上一个区块使用率正好是 50%，Base Fee 保持不变。  
* 如果上一个区块使用率正好满载 100%，Base Fee 会提高 12.5%。  
* 如果上一个区块使用率超过 50% 但小于 100%，Base Fee 会提高不到 12.5% 的某个值。  
* 如果上一个区块使用率是 0%，也就是空块，Base Fee 会降低 12.5%  
* 如果上一个区块不是空块且使用率没超过 50%，Base Fee 会降低不到 12.5%  
相对之前 Gas Price 完全由用户提供，经常大起大落，Base Fee 的变化显然是更加可预测也更加平缓的。每个区块的 Base Fee 是维护在区块头中的。  

我们假设0号区块的Base Fee是20，使用了70%的区块空间，那么下一个区块的Base fee是：
$r_{cur} := 20* (1+ \frac{1}{8} * \frac{s_{pred}-s_{target}}{1/2}) = 21$  
1号区块继续保持70%，则有如下图示：
<image src = "/docs/images/gas_fee_increase.png"></image>
因此引入该公式作为gas fee之后，整个base fee能够预测，对整个经济系统都有一定的把控。
根据参考论文所述，认为EIP-1559之所以这样设计是因为：  

1. 价格优先的拍卖策略，最优价格是依靠其他用户所提供的价格来参考的
2. 其他常见的拍卖设计，其中收费取决于包含的交易集合，例如二价（又称维克里）拍卖，可以通过虚假交易轻松被矿工操纵。

**广义二价拍卖（GSP）是一种针对多个物品的非真实拍卖机制。每个投标人都会出价。出价最高者获得第一名、第二高者、第二名，以此类推，最高出价者支付第二高者的出价，第二高者支付第三高者的出价，依此类推。**

3. 简单的费用估算，其中用户不需要考虑其他用户的行为，因此似乎需要一个基本费用——一种独立于当前区块中包含的交易而设定的价格。
4. 费用必须随着时间的迭代而迭代。
5. 区块的base fee必须销毁，要不然矿工就会伪造交易。
6. 当前区块容量很适合评估区块需求，因此选择动态区块能很好通过区块大小拟合需求进入动态调整。
7. tips能让矿工打包尽量多的区块，而不是空块。
8. 如果burn任何矿工的收益tips，那么会push交易再链下发生。

EIP-1559具备自动的寻找市场最佳价格的机制，因为过去需要用户自己推算，但是现在base fee是自动根据需求进行调整的，base fee会自动拟合到最佳base fee。

以太坊EIP-1559，使用了固定价格拍卖，取代了第一价格拍卖，这使得用户对于未来的Gas Fee可预测，支付的起就买，支付不起就暂时不买。区块空间俨然已经成为了一种经济学上的东西。每个人为区块空间可预测的付费。

### 关于eip-4844
EIP-4844引入了全新的交易格式Blob，这让以太坊的交易分为两种，一种是基于EIP-1559的普通交易（15M-30M ），一种的基于EIP-4844的Layer2交易（一个blob的大小是128KB，一笔交易可以携带两个blob，也就是258KB，每个块目标是3个blob (0.375 MB) 和最大6个 blob (0.75 MB)）。此 EIP 使每个信标块的带宽要求最多增加约 0.75 MB。这比当今区块的理论最大大小（30M Gas / 每个 calldata 字节 16 Gas = 1.875M 字节）大 40%。
目前，Layer2与普通用户在竞争极其有限的区块空间，但是随着未来Prague升级的引入，以太坊历史上第一次将有一个多维费用市场，为以太坊区块空间创建两种价格——**一个用于数据，一个用于执行**。两个现货市场将使用独立但相似的定价/拍卖机制。然而，考虑到数据块空间与执行块空间的消费者和使用情况的差异，两个市场之间可能会存在定价差异。以前layer2上传的数据存储在calldata内，现在存储在blob内。**calldata中每个0字节byte消耗4gas，非0字节每byte消耗16gas**，价格是线性的，且calldata 的大小是无直接限制的，根据layer2上传的数据来定价，当然会受制于天花板，就是区块的gas limit。
数据gas市场的gas价格机制如下所示：
$data_gasprice = MIN\_DATA\_GASPRICE * e**(excess\_data\_gas / DATA\_GASPRICE\_UPDATE\_FRACTION)$
其中，与 EIP-1559 一样，它是一个自我修正公式：随着超额量增加，其数量呈$data\_gasprice$指数级增长，减少使用量并最终迫使超额量回落。

### Prague的设计
我们的设计是基于以太坊相同的base fee + priority fee的经济模型。跳转->[txpool设计](/docs/research/txpoolDesign.md)


### 参考文档
[以太坊代币经济学设计机制](https://timroughgarden.org/papers/eip1559.pdf)
[以太坊代币经济学2022回顾](https://decentralizedthoughts.github.io/2022-03-10-eip1559/)  
[EIP-1559 常见问题解答](https://notes.ethereum.org/@vbuterin/eip-1559-faq)
[区块空间的思考](https://frontier.tech/ethereums-blockspace-future)  
[EIP-4844提案](https://eips.ethereum.org/EIPS/eip-4844)  
[Vitalik的EIP-1559论述](https://ethresear.ch/t/multidimensional-eip-1559/11651)  
[Proto-Danksharding FAQ](https://notes.ethereum.org/@vbuterin/proto_danksharding_faq#If-data-is-deleted-after-30-days-how-would-users-access-older-blobs)  

