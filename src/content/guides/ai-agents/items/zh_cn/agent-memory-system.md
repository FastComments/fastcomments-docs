Agent memory 是一个租户作用域的、**共享的** 键值池，租户中的每个 agent 都可以从中读取和写入。它的存在是为了让 agent 能在多次运行之间携带上下文。

### Why memory exists

LLM 的上下文是按运行划分的。没有 memory，向用户发出警告的 agent 下次看到相同用户时无法获知之前的警告。平台的升级策略 —— “在封禁前先警告” —— 依赖于 agent 能找到先前的警告。Memory 就是使这项功能可行的机制。

### Two kinds of memory

- **WARNING** - 在 [`warn_user`](#tool-warn-user) 流程中自动写入。agent 不会手动写入 `WARNING` 记录；它们是警告用户时的副作用。
- **NOTE** - 由 [`save_memory`](#tools-overview) 写入。agent 希望未来的 agent 知晓的一般上下文。

在决定是否应当封禁时，升级策略会专门查找 `WARNING` 记录。

### Tenant-scoped, agent-shared

你租户中的所有 agent 共享**一个 memory 池**。Agent A 保存的 note 对 Agent B 的 `search_memory` 调用是可见的。这是有意为之——你希望分诊 agent 的备注能为审核 agent 的决策提供信息。

`tenantId` 由执行器从 agent 自身的租户中设置 —— 绝不会来自 LLM 参数 —— 因此从构造上不可能发生跨租户的 memory 泄露。

### What's in a memory record

每条 memory 条目包含：

- **哪个 agent 写入的**，以及写入时间。
- **关于谁** —— 此 memory 所描述的用户。agent 无法伪造此信息；平台会根据触发 agent 的事件自动填充。
- **一个隐藏的关联帐号信号** —— 平台还会（私下）记录发源评论的 IP 指纹，以便未来的 memory 搜索可以显示来自相同 IP 的其他帐号的备注。该指纹永远不会展示给 agent 或 LLM。
- **备注本身** —— 最多 2000 字符的自由文本。
- **用于检索的标签** —— 最多 10 个短标签。
- **一种类型** —— 要么是 warning，要么是普通 note。
- **可选的评论链接** —— 如果 memory 与特定评论相关联。

### Search behavior

[`search_memory`](#tools-overview) 返回最多 25 条记录，按时间倒序排序，自动限定为（触发者的用户）或（触发者 IP 上的其他帐号）。结果在字符数上也有总量限制——所有返回内容合计最多 8000 字符；如果达到上限，会丢弃较旧的条目。

agent 不会传递 `userId` 或 `targetIpHash`。两者均由执行器设置。

### Persistence

Memory 没有 **TTL**。记录会一直保留，直到被显式移除。关于某用户的 WARNING 记录故意不会被自动删除 —— 如果无法长期找到升级历史，平台的“在封禁前搜索”检查就毫无意义。

移除 memory 的三种方式：

- 管理员删除了底层评论 —— 与该评论关联的任何 memory 会被级联删除。
- 用户被删除 —— 关于该用户的所有 memory 条目会在同一事务中被删除。
- 你的租户被删除。

目前没有用于删除单条 memory 记录的管理界面。

### Memory in dry-run

Dry-run 代理**不会**写入 memory。这是有意为之：dry-run 代理的假定决策不应污染共享的 memory 池。通过 `search_memory` 的读取在 dry-run 中正常工作 —— 代理可以看到来自真实 agent 的真实 memory —— 只是无法向其中添加内容。

### Memory in replays

与 dry-run 相同：回放代理不写入 memory。回放仅供预览。参见 [Test Runs (Replays)](#test-runs-replays)。

### Constraints summary

| Limit | Value |
|---|---|
| Memory content max length | 2000 chars |
| Memory tag max length | 64 chars |
| Memory tags max count | 10 |
| Memory query max length | 200 chars |
| Memory search result limit | 25 records |
| Memory search total content cap | 8000 chars |

### See also

- [工具：save_memory](#tools-overview) 用于写入。
- [工具：search_memory](#tools-overview) 用于读取。
- [工具：warn_user](#tool-warn-user) - 唯一会写入 WARNING 类型 memory 的工具。
- [工具：ban_user](#tool-ban-user) - 系统提示要求在此之前调用 `search_memory`。