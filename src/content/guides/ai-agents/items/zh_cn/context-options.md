The **Context** 部分位于编辑表单上，控制每次运行时代理接收多少信息。更多的上下文会产生更好的决策，但会提高每次运行的令牌成本，因此你只应包含代理实际需要的内容。

### What's always included

即使每个复选框都未选中，代理的上下文消息仍然包括：

- The **trigger event type**（例如 `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`）。
- The **page URL and URL ID**（在已知的情况下）。
- 触发运行的 **comment**（如果存在）——ID、author user ID、author display name、comment text、vote counts、flag count、spam/approved/reviewed flags、parent ID。作者的电子邮件**绝不会**发送给 LLM 提供商（PII 最小化）。
- 对于 `COMMENT_EDIT` 触发器的 **previous comment text**（以便代理可以比较修改前后）。
- 对于 `COMMENT_VOTE_THRESHOLD` 触发器的 **vote direction**。
- 触发操作的 **triggering user ID** 和 **badge ID**（用于版主徽章触发）。
- 当代理被允许授予徽章时，你的租户的 **badge catalog**（name、display label、description），以便代理可以在不需要你在提示中详细列出徽章的情况下选择合适的徽章。

所有不受信任的文本——评论正文、作者名、页面标题、指导文件本身——在上下文消息中都用类似 `<<<COMMENT_TEXT>>> ... <<<END>>>` 的标记进行**围栏**处理。平台的 system prompt 指示模型永远不要遵循这些围栏内的指令。这是平台的提示注入（prompt-injection）防御；你无需在你的提示中重复它。

### The three checkboxes

#### Include parent comment and prior replies in the same thread

添加：
- **parent comment**——ID、author、text。
- **Sibling replies**——同一线程中对相同父评论的先前回复。

适用于：任何在上下文中回应评论的代理（欢迎问候者、线程摘要器、在对话中阅读回复的版主）。

成本：小到中等。受限于给定线程上存在多少同级回复。

#### Include commenter's trust factor, account age, ban history, and recent comments

添加 **AUTHOR_HISTORY** 区块：

- 注册以来的 **Account age in days**。
- **Trust factor (0-100)**——FastComments 对该用户在本站点上的信任度评分。参见版主指南中的 [Spam Detection](/guide-moderation.html#spam-detection) 页面。
- 以前的 **ban count**。
- 在本站点的 **Total comments**。
- **Duplicate-content count**——如果用户最近发布了相同的文本（反垃圾信号）。
- **Same-IP cross-account signal**——来自同一 IP 在其他账户下的评论计数（备用账户信号）。IP 哈希本身**永远**不会发送给 LLM。
- **Recent comments**——该用户最多 5 条最近评论，每条截断为 300 个字符，并作为不受信任的文本围栏处理。

适用于：任何审 moderation 代理。没有这些信息时，模型会把新账户和长期良好行为的用户以相同的立场处理并封禁。

成本：中等。最近的评论会增加最多的令牌数。

#### Include page title, subtitle, description, and meta tags

添加 **PAGE_CONTEXT** 区块——title、subtitle、description，以及 FastComments 为该页面收集的任何 meta 标签。

适用于：欢迎问候者和线程摘要器，在这些情况下了解页面内容会显著提高输出质量。

成本：小。

### Community guidelines

第四个字段，**Community guidelines**，是一个免费文本的策略区块，包含在每次运行的 user-role 上下文消息中，并像评论正文和其他用户提供的内容一样作为不受信任的文本围栏处理。代理将其作为策略文本读取，但平台不会将其视为系统指令。有关应放入内容的说明，请参见 [Community Guidelines](#community-guidelines)。

### Adding context selectively

这些复选框按代理应用，而不是全局应用。一种常见的模式：

- 欢迎问候者：page context **开启**，thread context **关闭**，user history **关闭**。
- 版主：thread context **关闭**，user history **开启**，page context **关闭**。
- 线程摘要器：thread context **开启**，page context **开启**，user history **关闭**。

在代理实际执行的调用上，尽量只提供代理需要的最少上下文——额外的上下文会在每次运行时增加令牌成本，即使代理并未使用它。