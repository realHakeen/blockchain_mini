# 本章节主要用于架构设计

<image src = "/docs/images/prague_structure.png"></image>

根据MVP原则，我们从顶层到底层进行架构的设计，尽量的抽象、简洁、可拓展。
最上层的Node节点，是我们的用户主要的交互节点，然后底层的基础模块提供抽象的API给Node实例调用
底层，我们将其从功能方面进行划分，以依赖关系作为层级划分。
以下是各个模块的功能：

Consensus:负责共识模块，主要设计共识
Blockchain：负责维护该链
TransactionPool：负责维护交易池
Validation：负责验证交易
Account：负责维护全局账户状态
Miner：矿工


我们通过流程来分解架构中的各个角色，who do what：
## 运行节点Node实例
1. 运行节点
2. 创建私钥和地址，需要调用Account模块
3. 节点实例初始化blockchain、transaction pool
4. 通过p2p，节点调用同步功能api，同步blockchain、transaction pool，每个slot同步一次（暂时设置为12s）
5. 后台运行

## 交易
1. Node实例发起一笔交易（miner模块负责实际发起），验证模块验证交易合法性（签名，格式），miner放入transaction pool
2. 设定12s为一个slot，miner打包交易成一个block，并且执行所有的block，改变account模块的状态，
3. node实例调用consensus模块进行block确认
4. consensus发起节点投票，将block 唯一确认后，广播给其它peer
5. block变成saledblock，放入blockchain中形成canonical chain


## Primitives specs
首先，我们的block内是由交易组成的，transaction为了MVP原则，因此简洁，目前仅仅只有转账功能，  
|  字段  |  描述  |  规格  ｜
|  ----  | ----  |  ----｜
| from  | 交易发起sender |  address, U256|
| to  | 交易接收方 | address, U256｜
| amount  | 转账数量 | 18位，U256 ｜
｜base gas｜基本gas 是系统自动设置｜｜
|  data | 携带数据 | 最多20个字节 ｜
｜signature｜交易签名｜｜




我们抽象出来各个模块需要提供的接口以及其功能：