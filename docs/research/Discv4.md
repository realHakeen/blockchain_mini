<!--
 * @Author: realHakeen yhk15802841343@gmail.com
 * @Date: 2023-08-13 15:54:16
 * @LastEditors: realHakeen yhk15802841343@gmail.com
 * @LastEditTime: 2023-08-18 15:58:14
 * @FilePath: /blockchain_mini/docs/research/Discv4.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
# Kademila
我们前面有论述过，Discv4是执行层的寻找节点的方案。而Discv4是源于Kademila + Ethereum's node discovery protocol进行搭建的。
这个是一个什么协议呢？Kademila是一个分布式的哈希表，它specifies了网络的结构和信息交换的规则。他们使用UDP进行信息的交换。每个节点都有Node ID，不仅仅是identification，Kademila也用node ID定位values，相当于也是key。

In order to look up the value associated with a given key, the algorithm explores the network in several steps. Each step will find nodes that are closer to the key until the contacted node returns the value or no more closer nodes are found. This is very efficient: like many other DHTs, Kademlia contacts only $O(logn)$ nodes during the search out of a total of n nodes in the system.

[I2P](https://en.wikipedia.org/wiki/I2P)'s implementation of Kademlia is modified to mitigate Kademlia's vulnerabilities, such as Sybil attacks.

## System Details
Kademlia uses a **distance** calculation between two nodes. This distance is computed as the exclusive or (XOR) of the two node IDs, taking the result as an unsigned integer number. Keys and node IDs have the same format and length, so distance can be calculated among them in exactly the same way. The node ID is typically a large random number that is chosen with the goal of being unique for a particular node (see UUID). It can and does happen that geographically far nodes – from Germany and Australia, for instance – can be "neighbors" if they have chosen similar random node IDs.


**Kademlia是一种点对点分布式哈希表（DHT），它在容易出错的环境中也具有可证明的一致性和性能。使用一种基于异或指标的拓扑结构来路由查询和定位节点，这简化了算法并有助于证明。该拓扑结构有一个特点：每次消息交换都能够传递或强化有效信息。系统利用这些信息进行并发的异步查询，可以容忍节点故障，并且故障不会导致用户超时。**

kademlialib：一个用Rust实现的高性能且可扩展的Kademlia DHT库。


## KAD算法要处理的问题
1. 如何分配存储内容到各个节点，新增/删除内容如何处理
2. 如何找到存储文件的节点/地址/路径

## 节点状态
节点的基本属性包括如下：
1. Node ID
2. 节点IP地址和端口号
在 Kad 网络中，所有节点都被当作一颗二叉树的叶子，并且每一个节点的位置都由其 ID 值的最短前缀唯一的确定。
这些性质使得异或运算可以作为一种距离度量函数，它可以衡量节点和数据之间的相似性和差异性。越相似的节点或数据，它们的ID之间的异或距离越小；越不相似的节点或数据，它们的ID之间的异或距离越大。这样，我们就可以根据ID来确定节点和数据之间的相对位置和存储位置。这个只是逻辑上最近，并不一定是实际地理上最近。实际地理上最近的节点可能并不是逻辑上最近的节点，因为ID是随机生成的，并不与地理位置相关。这可能会导致网络效率降低，因为距离远的节点之间通信需要更多的时间和资源。为了解决这个问题，有些Kademlia的实现方式会考虑地理位置因素来优化ID的生成和选择。例如，在选择路由表中存储哪些节点信息时，可以优先选择地理位置较近的节点；在生成新节点的ID时，可以参考已知节点的地理位置信息来生成一个更接近自己地理位置的ID。这样可以提高网络效率和稳定性。

# Discv4 Specs

## 节点ID
每个节点都有一个加密身份，即secp256k1椭圆曲线上的密钥，节点的公钥用作其标识符或“节点ID”。
每个节点之间的距离是“公钥”哈希值上的按位异或，以数字形式表示。

```
distance(n₁, n₂) = keccak256(n₁) XOR keccak256(n₂)
```

## 节点记录
发现协议的参与者应维护包含最新信息的[节点记录（ENR）](https://github.com/ethereum/devp2p/blob/40ab248bf7e017e83cc9812a4e048446709623e8/enr.md)。
ENRRequest：请求本地记录
FindNode：Kademila查找，找到后发送ENRRequest从Response中返回记录

## KAD Table
发现协议中的节点保存，k = 16，每当遇到新的节点N₁时，就可以将其插入到相应的桶中。如果桶包含的条目少于kN₁，则可以简单地将其添加为第一个条目。如果存储桶已包含条目，则需要通过发送Pingk数据包来重新验证存储桶中最近最少出现的节点 N2 。如果没有收到来自 N2 的回复，则认为它已死亡，将其删除并将 N1 添加到存储桶的前面。

## ENR：以太坊节点记录
以太坊节点记录（ENR）是一个包含三个基本元素的对象：签名（根据某种商定的身份方案生成的记录内容的哈希值）、跟踪记录更改的序列号以及 key:value 的任意列表对。这是一种面向未来的格式，可以更轻松地在新对等点之间交换识别信息，并且是以太坊节点的首选网络地址格式。

## 为什么建立在UDP？

UDP 不支持任何错误检查、重新发送失败的数据包或动态打开和关闭连接 - 相反，它只是向目标发出连续的信息流，无论是否成功接收。这种最小的功能也转化为最小的开销，使得这种连接非常快。对于Discover，节点只是想让其存在为人所知，以便随后与对等方建立正式连接，UDP 就足够了。然而，对于网络堆栈的其余部分，UDP 不适合其用途。节点之间的信息交换相当复杂，因此需要一个功能更齐全的协议来支持重发、错误检查等。与 TCP 相关的额外开销值得额外的功能。

# 参考资料
[Kademila维基百科](https://en.wikipedia.org/wiki/Kademlia)  
[Ethereum Discv4 Specs](https://github.com/ethereum/devp2p/blob/40ab248bf7e017e83cc9812a4e048446709623e8/discv4.md)  
[DHT](https://codethechange.stanford.edu/guides/guide_kademlia.html)  
