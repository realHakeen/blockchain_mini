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

那么整个公式就是：

$r_{cur} := r_{pred}* (1+ \frac{1}{8} * \frac{s_{pred}-s_{target}}{s_{target}})$

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
目前，Layer2与普通用户在竞争极其有限的区块空间，但是随着未来Dencun升级的引入，以太坊历史上第一次将有一个多维费用市场，为以太坊区块空间创建两种价格——**一个用于数据，一个用于执行**。两个现货市场将使用独立但相似的定价/拍卖机制。然而，考虑到数据块空间与执行块空间的消费者和使用情况的差异，两个市场之间可能会存在定价差异。以前layer2上传的数据存储在calldata内，现在存储在blob内。**calldata中每个0字节byte消耗4gas，非0字节每byte消耗16gas**，价格是线性的，且calldata 的大小是无直接限制的，根据layer2上传的数据来定价，当然会受制于天花板，就是区块的gas limit。
数据gas市场的gas价格机制如下所示：  

data_gasprice = MIN_DATA_GASPRICE * e**(excess_data_gas / DATA_GASPRICE_UPDATE_FRACTION)

其中，与 EIP-1559 一样，它是一个自我修正公式：随着超额量增加，其数量呈$data\_gasprice$指数级增长，减少使用量并最终迫使超额量回落。

### 以太坊的质押收益
以太坊的POS质押收益取决于以下几个因素：

* 质押的ETH数量：每个验证者需要质押至少32个ETH，才能参与共识层的验证和出块，以获得奖励。  
* 网络中的总质押量：网络中质押的ETH越多，每个验证者的收益率就越低，反之亦然。这是为了平衡网络的安全性和可用性。  
* 验证者的在线时间和行为：验证者需要保持在线并按照协议规则进行验证和出块，否则会受到惩罚，损失部分或全部质押的ETH。如果验证者长时间离线或作恶，可能会被逐出网络，失去质押资格。  
* 网络的利率参数：网络根据总质押量和验证者的在线率，动态调整每个区块的基础奖励和惩罚。这些参数旨在保持网络的激励机制和经济模型。  

### POS收益
POS收益来自于代币的额外释放，我们来看看以太坊的代币释放机制如何设计的。以太坊需要32个ETH才能进行质押，但是[第111次共识层会议](https://www.youtube.com/watch?v=ybgQuRcz9sg&ab_channel=Ethereum)有提议将门槛提高到2048个，那这会极大的降低去中心化程度，一定程度降低安全性，但是好处是门槛提高意味着网络的沟通成本下降，整体网络性能将变好，导致网络拥堵问题得到一定的解决，但是其收益与损失相比，我们无从得知。
在区块链网络中，epoch是决定某些事件何时发生的一段时间。例如，奖励分配的速率或何时**分配一组新的验证器来验证交易**。使用纪元的区块链协议因定义纪元的时间段而异。对于 PoS 以太坊，每 32 个时隙（1个slot就是一个出块时间12s，32个slot就是6.4 分钟）发生一个纪元(也就是每32个slot会更改一次委员会)。
epoch中的每个时隙代表验证者委员会（至少 128 名验证者组成的组）的设定时间，以提议并证明（投票）新区块的有效性。每个时期有32组委员会。当一个委员会被分配到一个区块后，从该委员会的 128 名成员中随机选出一名人作为区块提议者。该人是唯一可以提出新交易区块的人，而其他 127 人对该提案进行投票并证明交易。
一旦多数人同意，该区块就会被添加到区块链中，并且提出该区块的验证者会根据公式计算收到可变数量的 ETH 。因为真实的Ethereum的收益包括了POS+MEV收益，因此很难进行计算，我们只进行基本的POS公式计算。
$base\_reward = effective\_balance * (\frac{base\_reward\_factor} {(base\_rewards\_per\_epoch * sqrt(sum(active\_balance)))})$
我们来讲解一下：
其中，$base\_reward\_factor$ 是 64，$base\_rewards\_per\_epoch$ 是 4，$sum(active balance)$ 是所有活跃验证者的质押以太币总数。
这意味着基础奖励与验证者的有效余额成正比，与网络中的验证者数量成反比。 验证者越多，整体发行量越大（如 $sqrt(N)$），但每个验证者的 $base\_reward$ 越小（如$1/sqrt(N)$）。 这些因素影响质押节点的年化利率。
总奖励由五个部分之和组成，每个部分都有一个权重，决定每个部分在总奖励中的比重。 这些部分是：
1. 来源投票：验证者给正确的来源检查点进行了及时投票  
2. 目标投票：验证者给正确的目标检查点进行了及时投票  
3. 头部投票：验证者给正确的头部区块进行了及时投票  
4. 同步委员会奖励：验证者参与了同步委员会  
5. 提议者奖励：验证者在正确的时隙提议了区块  

来源检查点和目标检查点是以太坊用于实现最终确认性的机制，来源检查点是验证者投票时认定的上一个时段的检查点，目标检查点是验证者投票时推举的新的时段的检查点。如果一个检查点得到了 **2/3 的总余额**支持，那么该检查点就被合理化了。如果一个合理化的检查点的下一个时段的检查点也被合理化了，那么前一个检查点就被敲定了。
每个部分的类型以及其权重，如下所示：  
* TIMELY_SOURCE_WEIGHT    uint64(14)  
* TIMELY_TARGET_WEIGHT    uint64(26)  
* TIMELY_HEAD_WEIGHT  uint64(14)  
* SYNC_REWARD_WEIGHT  uint64(2)  
* PROPOSER_WEIGHT uint64(8)  

这些权重加起来等于 64。 奖励的计算方法是适用权重的总和除以 64。 如果验证者及时给来源、目标和头部投票，提议一个区块以及参与同步委员会，他们就能获取 $64/64 * base\_reward == base\_reward$。 然而，验证者通常不是区块提议者，所以它们的最大奖励是 
$\frac{(64-8)}{64} * base\_reward == \frac{7}{8} * base\_reward$
既不是区块提议者，也不参与同步委员会的验证者能收到 
$\frac{(64-8-2)}{64} * base\_reward == \frac{6.75}{8} * base\_reward$
当然还有其他的激励激励快速进行验证，比如$ inclusion\_delay\_reward $，我们不与赘述。

### Prague的设计
我们的设计是基于以太坊相同的base fee + priority fee的经济模型。跳转->[tx的生命周期](/docs/research/tx_life_time.md)，然后需要创建一个原生代币，PRA，作为我们的支付代币。

### PRA代币的经济模型
这是一个比较复杂的模块。需要解决以下几个问题：
* 现有硬币数量是多少？还会添加多少？  
* 供给是通货膨胀（增加）还是通货紧缩（减少）？  
* 这些硬币是否有实用性，即除了交换之外还可以用于其他用途吗？  
* 现实世界的用例是什么？  
* 谁拥有大部分代币？它是分散的还是集中在少数几个账户中？  

众所周知，btc是每四年减半一次，价格就会发生飙升，这个是周期性的经济模型导致的，而以太坊通过技术创新、代币销毁、使用率增加、POS等手段来让代币价格上涨，这是两种不同的代币刺激手段。  

Prague选择使用最基本的Base Fee + Priority Fee进行经济模型的运转，我们的代币为PRA，初始数量为10，000，000PAR，位数是18位，最小单位是$mini$，Gas Fee的基本单位是$G$，$1G = 1 * 10^9 mini$。

打包交易的矿工会得到Priority Fee，而Base Fee会被销毁，PRA唯一会生产的渠道就是给POS共识节点进行质押，维护网络共识获得的收益。因此我们的系统也分拆解成两部分，一个是执行层负责打包交易，预执行状态，生成区块。另一个是共识层，负责sync，完成区块上链。整体PRA的代币数量是根据生态系统内使用数量而定，越多人使用销毁的Base Fee就越多，那么当Base Fee大于区块质押能产生的PRA，则整个系统的经济模型陷入通缩。

由于是demo，因此我们设计，刚开始运行全节点，注册地址的用户，将能够获得1000枚PRA，作为POS或者提交交易的Gas Fee。
Base Fee，最低为8。Base Fee的上涨遵循Ethereum的公式，
$r_{cur} := 20* (1+ \frac{1}{8} * \frac{s_{pred}-s_{target}}{1/2}) = 21$  

为了轻量，我们直接使用以太坊的这个最基本的公式：
$base\_reward = effective\_balance * (\frac{base\_reward\_factor} {(base\_rewards\_per\_epoch * sqrt(sum(active\_balance)))})$

### 惩罚验证者
如果验证者行为不诚实或离线，则会受到处罚。例如，提出多个区块（模棱两可）或提交矛盾的证明（投票）会导致称为削减的惩罚，这意味着验证者会损失一定比例的质押ETH。被削减的以太币数量取决于同时被削减的验证者数量，也称为“相关性惩罚”。其范围可以从单个验证者的 1% 到验证者权益的 100% 被削减。



### 参考文档
[以太坊代币经济学设计机制](https://timroughgarden.org/papers/eip1559.pdf)
[以太坊代币经济学2022回顾](https://decentralizedthoughts.github.io/2022-03-10-eip1559/)  
[EIP-1559 常见问题解答](https://notes.ethereum.org/@vbuterin/eip-1559-faq)
[区块空间的思考](https://frontier.tech/ethereums-blockspace-future)  
[EIP-4844提案](https://eips.ethereum.org/EIPS/eip-4844)  
[Vitalik的EIP-1559论述](https://ethresear.ch/t/multidimensional-eip-1559/11651)  
[Proto-Danksharding FAQ](https://notes.ethereum.org/@vbuterin/proto_danksharding_faq#If-data-is-deleted-after-30-days-how-would-users-access-older-blobs)  
[代币经济学设计](https://tokenomicsdao.xyz/blog/tokenomics-101/tokenomics-101-bitcoin-ethereum/)
[质押收益计算](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/rewards-and-penalties/)  
[Serenity Design Rationale by Vitalik](https://notes.ethereum.org/@vbuterin/serenity_design_rationale?type=view#Serenity-Design-Rationale)  
