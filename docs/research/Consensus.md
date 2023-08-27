# POS机制
实际上，POS只是一类共识机制的统称，实际上以太坊的使用的细分共识机制是[Casper FFG](https://eth2book.info/capella/part2/consensus/casper_ffg/)。
Casper FFG 的有效性实际上取决于两个重要想法。首先，两阶段提交（论证和最终确定），第二，负责任的安全。
两阶段提交为 Casper FFG 提供了经典的共识安全性。它可以将区块声明为最终区块，并确保诚实的验证者不会恢复它们。但只有当超过三分之二的股份由诚实的验证者控制时，这一点才可执行，这是我们不能总是假设的。
除此之外，Casper FFG 还为超过三分之一的验证者不诚实的情况提供了称为“责任安全”的额外保证。如果该链遇到最终性冲突，则质押以太币总量的至少三分之一将被烧毁。这是通过削减违反两条 Casper 戒律之一的验证器来强制执行的。

## Casper FFG
Casper FFG 是一种元共识协议。它是一个覆盖层，可以在底层共识协议之上运行，以增加最终确定性。最终性是指链中的区块保证永远不会被还原：它们将永远成为链的一部分。在以太坊的权益证明共识中，底层协议是LMD GHOST，它不提供最终性——验证者总是有可能决定建立一条竞争链，而且这样做不会受到真正的惩罚。Casper FFG 充当“最终性小工具”，我们用它来为 LMD GHOST 添加最终性。
这意味着当我们看到诚实验证者的多数票时，我们可以使用计票来判断。更准确地说，来自管理大部分权益的验证者的投票 - 在接下来的所有内容中，每个验证者的投票都根据其管理的权益的价值进行加权。
FFG 部分代表“Friendly Finality Gadget”

## Details
# overview

Casper FFG 受到 PBFT 的启发，可以看作是后者共识协议的改进。它继承了PBFT的核心设计，同时添加了新的机制并简化了一些规则。

vitalik发现，只需要四个规则就能实现共识的良好运行，一旦违反任意一个就会导致押金被大幅削减。

1. 发送COMMIT之前需要2/3的PREPARE
2. 如果在某个EPOCH中进行PREPARE，指向某个特定的先前EPOCH，那么需要在该EPOCH中看到2/3的PREPARE，并且这些PREPARE必须指向相同的先前EPOCH。
3. 如果在某个epoch内进行了COMMIT，那么可以清楚的看到该时期内有2/3的PREPARE
4. 不能在一个epoch内PREPARE两次

这四条规则可以进一步减少为两条：

<aside>
💡 验证者不得对同一目标高度发布两次不同的投票、验证人不得在其其它投票范围内进行投票

</aside>

# How it works

<image src = "/docs/images/CasperFFG.png"></image>  

checkpoints：来自underlying chain提交的block就是checkpoints。

Casper FFG中，checkpoint是指在区块树中特定高度的区块，它们用于在PoW和PoS之间建立一个桥梁，实现区块的最终确定性。每个checkpoint之间相隔N个区块，每个checkpoint需要获得超过2/3的验证者的投票才能被认可。当一个checkpoint被认可后，它就成为justified，当一个justified的checkpoint收到它的子节点的超过2/3的投票后，它就成为finalized。finalized的checkpoint以及它之前的所有区块都不会被回滚，从而保证了系统的安全性和活跃性。

每经过两轮投票，每个checkpoint都会变得“justified”然后变成“finalized”。r/b1/b2是finalized，b3是justified。必须要在同一链路上，后边的b3大到了justified，那么b2就能从justified变成finalized。

节点应该选择哪些检查点进行投票呢？每个节点必须遵循**分叉选择规则**来选择下一个要链接的检查点。在Casper FFG中，规则是选择最高的“合理”检查点来建立链路。因此现在以太坊是使用POW进行区块挖掘，POS进行区块的最终性确认。检查点是每32个区块设置一个。

**是不是就是说明以太坊的出块时间 12s * 50 个区块，最少这么长时间才能说明区块是finalized的？才能保证呢？**

不是的！以太坊的区块确认并不是简单地按照区块数量来计算的，而是要考虑区块的权重。权重是由区块的难度和叔块的数量决定的，难度越高，叔块越多，权重越大。

以太坊使用了一种叫做Ghost协议的算法，它可以让区块链更快地达成共识，同时也减少了分叉的浪费。Ghost协议的核心思想是，当有两个或多个分叉出现时，不仅要看哪个分叉的区块数量更多，还要看哪个分叉的总权重更大。总权重是指分叉上所有区块和叔块的权重之和。因此，有可能出现这样一种情况，即一个分叉上的区块数量少于另一个分叉，但是由于它的区块难度更高，或者包含了更多的叔块，它的总权重反而更大，从而成为主链。这样就可以让网络更快地达成共识，而不用等待很多个区块才能确认。

所以，以太坊的区块确认时间并不是固定的，而是根据网络状况和区块权重动态调整的。一般来说，一个区块被合理化（justified，即获得总质押以太币 2/3 的投票）后就可以认为是安全的，但是如果网络出现分裂或攻击，可能需要等待更多个区块才能确保不被回滚。

**因此以太坊的共识机制叫做Casper，是由Casper FFG与Ghost协议共同形成的，Casper FFG用于确定区块；而Ghost用于避免分叉的产生。**

在 Gasper 中，原来的分叉选择规则被弃用，转而采用一种更复杂的算法，即 LMD-GHOST。 请务必了解，在正常情况下，分叉选择规则是不必要的 - 每个时隙都有一个区块提议者，并有诚实的验证者证明此区块。 只有在网络异步性较大或不诚实的区块提议者模棱两可的情况下，才需要分叉选择算法。 然而，当这些情况确实出现时，分叉选择算法是确保正确链的关键防御。

关于可以最终确认的时间，就是justified时间（12（s/slot） * 32（slot/epoch） = 6.4mins）+ 到达checkpoint的时间（一般最长是32个block = 12s * 32 = 6.4mins）这个时间已经包含了中间32个区块的出块时间，因为是并发的。

## 以太坊的共识规范
[这里是以太坊的共识规范](https://github.com/ethereum/consensus-specs)，我们能够查到每一次升级的共识层的设计。  

<image src = "docs/images/ethereum_consensus_design_process.png"></image>

这里陈列着所有以太坊共识层的升级规划以及进度。以上图片是已经完成了的升级，目前正在进行Deneb的共识层升级。



# 参考资料
[Casper vs Tendermint](https://blog.cosmos.network/consensus-compare-casper-vs-tendermint-6df154ad56ae)  
[Casper FFG](https://medium.com/unitychain/intro-to-casper-ffg-9ed944d98b2d)  