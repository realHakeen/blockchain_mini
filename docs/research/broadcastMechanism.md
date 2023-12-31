### 传播机制
以太坊的传播机制分为Mempool内的tx广播、区块广播、同步，三种需要广播的类型。以太坊有专门的[广播协议](https://github.com/ethereum/devp2p/blob/master/caps/eth.md)，以让不同客户端和定制化策略客户端节点能够进行有效通信。

### 基本操作
一旦建立连接，就必须发送状态消息。收到对等方的状态消息后，以太坊会话处于活动状态，并且可以发送任何其他消息。其实以太坊的整个P2P交流时比较复杂的。我们一步一步进行分解。

### 对等节点
以太坊的对等节点（Peer nodes）是指参与以太坊网络的节点，它们相互连接并共享信息和数据。在以太坊区块链中，每个节点都是网络的一部分，并且通过点对点（P2P）协议进行通信。
这些对等节点负责维护以太坊的分布式性质，共同参与区块链的交易验证和区块生成过程。每个对等节点都存储着完整的区块链副本，并通过网络将新的交易和区块传播给其他节点。
当一个以太坊节点需要发送交易或查询区块链状态时，它可以向连接的对等节点发送请求。这些请求被广播到整个网络，从而确保信息在整个网络中传播和同步，使得所有节点的数据保持最新和一致。
以太坊的对等节点不仅包括全节点（Full Node）——完整地存储和维护整个区块链，还包括轻节点（Light Node）——仅存储区块头部信息，并且通过与其他节点交互来查询数据。轻节点可以更节省存储空间和带宽，但相对来说对于网络中的状态查询可能更加依赖其他节点。
总的来说，以太坊的对等节点共同构成了一个去中心化的网络，每个节点都有着相同的权利和责任，从而确保了以太坊区块链的安全性、可靠性和去中心化特性。

当一个新节点加入以太坊网络时，它需要找到一些已知的对等节点以建立连接，并开始参与网络中的通信和数据传输。这个过程通常包括以下几个步骤：

1. 初始节点列表：**新节点可以通过预先配置的一组初始节点列表来尝试连接到网络**。这些节点是已知的、可信任的节点，可以帮助新节点加入网络。初始节点列表通常由以太坊客户端软件提供，并且可能随着版本更新而有所改变。
2. DNS发现：一些以太坊客户端支持通过域名系统（DNS）进行节点发现。在这种情况下，新节点可以通过域名查找以太坊网络的对等节点列表。
3. 本地网络：新节点还可以尝试在本地网络中查找其他对等节点。例如，如果新节点位于局域网内，它可以扫描本地网络并尝试连接到其他运行在同一网络上的节点。
4. 节点广播：新节点可以发送特定的节点发现请求到已知节点，要求它们告知新节点其他对等节点的存在。已知节点可以回复一个包含其他节点信息的消息，从而帮助新节点建立更多连接。
5. 网络浏览器：一些以太坊网络提供网络浏览器，允许用户查找当前活跃的节点列表。新节点可以通过网络浏览器查找其他对等节点，并尝试连接到它们。
在连接到**一个或多个对等节点后**，新节点就可以开始与网络中的其他节点交换数据，并逐渐扩大其对等节点列表。这种方式使得新节点可以快速地与整个以太坊网络建立连接，成为网络的一部分，并开始参与区块链交易验证和数据传输过程。

### 对等节点的数量
在以太坊网络中，节点可以连接到多个对等节点，但实际连接数量可能因节点类型、网络拓扑结构和网络条件而有所不同。以下是一些常见的节点连接数量情况：
全节点（Full Node）：全节点通常连接到较多的对等节点，通常在数十个到数百个之间。这是因为全节点需要存储和维护整个区块链的完整副本，连接到更多的节点可以帮助其更快地接收新的交易和区块，并加快数据同步的速度。
轻节点（Light Node）：轻节点通常连接到较少的对等节点，通常在几个到十几个之间。轻节点不需要存储完整的区块链，它们只存储区块头信息，并通过与其他节点交互来查询数据。较少的连接数对轻节点来说已经足够满足数据查询的需要。
* Geth 和 Parity 节点：Geth 和 Parity 是两种常见的以太坊客户端软件。它们在默认情况下都允许最大连接数为 25 个。这个数值可以根据需要进行调整，但连接到太多的节点可能会导致网络流量增加和资源消耗增加。
* 其他因素：节点连接数量还可能受到网络带宽、硬件资源和网络拓扑结构等因素的限制。有些节点可能连接到特定的对等节点，例如矿池会与自己的矿工节点建立更多的连接，以便更有效地组织区块的挖掘。
* 总的来说，节点的连接数量通常是一个权衡之间。连接到更多的节点可以提高网络的去中心化程度和可靠性，但同时也会增加网络流量和资源消耗。节点运营者需要根据自己的需求和网络条件，选择适当的连接数量来维护节点的正常运行和参与以太坊网络的顺畅通信。

### libp2p和devp2p
在合并之前，以太坊仅使用devp2p ，一个专用的网络堆栈和一组网络协议，在某些方面与 libp2p 没有什么不同。尽管以太坊和 IPFS/libp2p 社区之间进行了谈判，希望采用一种解决方案而不是两种解决方案，但时机并不合适，以太坊附带了 devp2p 作为其解决方案。之后经过与protocol Labs的合作，共同开发了Gossipsub作为以太坊的传播协议，并且其成为了libp2p的协议栈之一，并且作为补充还有一个Discv5作为节点的发现协议。就是所有的工作都需要对于的标准和规范，我们才知道如何为以太坊建造客户端。
节点使用的具体p2p协议的协议栈包括如下：

* Transport : TCP  
* Encryption : Noise  
* Protocol Negotiation : Multistream Select 1.0
* Stream Multiplexing: mplex or yamux
* Messaging: GossipSub v1.1
* Finding Nodes : Discv5  
在[这个Github文档](https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/p2p-interface.md#design-decision-rationale)中，描述了为什么选择这些协议而不是其它替代品。  
接下来我们将概述以太坊的传输协议栈。  

### libp2p
libp2p（“库对等”的缩写）是一个对等 (P2P) 网络框架，支持 P2P 应用程序的开发。它由协议、规范和库的集合组成，促进网络参与者（称为“对等点”）之间的 P2P 通信。P2P 网络是去中心化的，这意味着参与者在相对“平等的基础上”直接相互沟通。没有中央服务器或机构控制网络。P2P 网络不需要一组与“客户端”行为不同的特权“服务器”，如在主要的 客户端-服务器模型中那样。P2P 网络可以采取多种形式，包括BitTorrent等文件共享系统 、比特币 和以太坊等区块链网络以及Matrix等去中心化通信标准 。这些系统都有不同的挑战和权衡，但它们的共同目标是改进传统的客户端-服务器网络模型。  

<image src = "/docs/images/libp2p_stack.png"></image>  

其实P2P系统面临的第一步问题就是寻找对等点问题。
关于具体的rust实现的libp2p在这里可以找到->[libp2p for rust](/docs/research/rust-libp2p.md)  




### Prague的通信设计
由于以太坊作为一个完整项目，其设计庞杂，我们并不会实现以太坊的全部细节，但是我们会使用rust-libp2p来进行节点之间通信协议的搭建。





参考资料：  
[以太坊交易如何传播](https://www.alchemy.com/overviews/transaction-propagation)  
[以太坊广播协议](https://github.com/ethereum/devp2p/blob/master/caps/eth.md)  
[以太坊权益证明共识规范](https://github.com/ethereum/consensus-specs/tree/dev)  
[网络层](https://ethereum.org/pt/developers/docs/networking-layer/)  
[libp2p与以太坊的历史渊源](https://blog.libp2p.io/libp2p-and-ethereum/)  

