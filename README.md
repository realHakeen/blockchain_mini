## 简述
<image src = "./docs/images/LOGO.png"></image>
这条链是一个简单的区块链mini demo，我们尝试使用Rust语言进行区块链的构建，我们给一个简单的名称就定为以太坊既DENCUN升级后的下一次执行层升级的名字——Prague。

该开始只是一条最简单的链，相当于是作为我们练习Rust+区块链开发的第一个作品，我们是一点点扩充的，设计是**从顶层至底层的**，就是一个想法开始，因此我是没有明确的ddl的，因为在搭建的过程中，会发现有很多细节需要进行研究，我们还将在docs文档里面介绍我们的想法以及在coding过程中需要扩充的东西以及其研究，我们尽量**模块化**以方便后续的扩充，我们所有的研究都是基于以太坊的现行方案。
## 初始架构
首先，我进行了基本研究。每一条区块链都是一笔交易去触发，然后需要一个block，一个blockchain等struct。以下是最基本的架构以及设计的原因：

通过一个用户的角度来审视区块链的架构：
用户发起一笔交易，进入交易池，矿工挑选交易，打包成区块，然后矿工将区块，矿工收入上一个区块的hash值，完成整条链，然后广播区块。这一句话实际上相当不严谨，里面有太多东西需要思考。

关于，这个流程里面什么时候
1. 交易、区块需要哪些字段
2. 该使用什么哈希函数
3. 交易池子如何进行组织
4. 交易如何组织
5. 哪个矿工有资格获得区块的出块权？有没有奖励？区块内交易达到多少能形成一个完整区块？
6. 交易如何广播给其他节点
7. 如何将区块同步给其它节点呢
8. 底层使用的数据库以及如何设计


首先，我们的交易包含的字段设计：
刚开始，我们设计：（我们的交易目前仅仅支持转账）
1. from：该交易的发起人
2. to：该交易的接受人
3. amount：转账的数量
4. fee 转账需要的费用（为了简洁和可拓展性我们暂时以fcfs）
5. data：转账携带的数据


我们设计一个全局状态，是一个KV对，里面包含了：
1. 账户余额
2. 链上的数据data字段

我们设计一下block：矿工打包多少个交易即可呢？不可能一直打包交易没有上限吧，因此我们设计两个
1. 数量到达：当转账的tx数量达到至多100条则打包成一个块。
2. 时间到达：当时间大约在12s左右时，我们打包成一个块。
相关研究在：[以太坊的区块设计](https://ethereum.org/en/developers/docs/blocks/)

block的字段设计：
1. BlockHash 用来记录区块的整个哈希值
2. presentHash 上一个区块的哈希值
3. BlockHeight/blocknumber区块高度 因为区块链是使用链式结构，因此我们选择区块高度
4. txns 交易的数量
5. txRoot 所有交易的根节点哈希值
6. stateRoot 当前区块快照后的全局状态的stateRoot
7. Timestamp 时间戳，区块生成的时间，具体是在区块初始化的那一刻的Unix时间
8. BlockReward 该区块生产出来给矿工的费用
9. Size 该区块占据的存储空间大小
10. Data 矿工的数据，当前不对其进行约束，矿工任意写

矿工的流程图如下：
<image src = "./docs/images/tx.png"></image>
对于worker来说：
监听，从网络上收到tx，然后将tx放入到自己的subpool中，再从subpool中按照规则选取txs，然后将tx进行运算，预修改全局状态，也就是账户的余额，形成区块打上时间戳，发送到网络上，之后通过节点之间的共识机制，达成共识，上链，所有节点下载并且同步。

1. 这里有许多衍生的问题，谁来当Worker，Worker如何选择？
2. 共识节点如何选择？
3. 共识如何设置？
4. Txpool的设计，每个节点都维护着自己的Subpool，Subpool之间如何互通？

## 机制研究
我们一个一个来解决：  
[该使用什么哈希函数？](./docs/research/chooseHashFunc.md)  
[地址的生成——ECDSA函数](./docs/research/ECDSA.md)  
[以太坊的代币经济学](./docs/research/tokennomics.md)  
[交易的生命周期](./docs/research/tx_life_time.md)  
[以太坊执行层消息传递协议DevP2P](/docs/research/Devp2p.md)  
[执行层设计](./docs/research/network.md)  
[共识层设计](./docs/research/Consensus.md)  
[矿工机制的设计:Worker的选择](./docs/research/workerMechenism.md)  
[Casper FFG共识机制以及共识节点的选择](./docs/research/Consensus.md)    
【WIP】[底层数据库的设计](./docs/research/DataBase.md)  
[项目架构设计](./docs/research/structureDesign.md) 







