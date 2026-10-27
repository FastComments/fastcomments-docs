---
一个 **AI Agent** 是一个自主工作者，受限于您的 FastComments 租户，监视社区中的事件并代表您采取行动。

每个代理都有三项您可以控制的内容：

1. **A personality.** 一个自由文本的初始提示，定义语气、角色和决策风格（例如“You are a warm community greeter”, “You enforce the community rules but err toward warning over banning”，等等）。
2. **One or more triggers.** 一组触发条件的列表，使代理醒来——新评论、评论达到投票或标记阈值、版主操作、用户在站点的首次评论等。完整列表请参见 [Trigger Events Overview](#triggers-overview)。
3. **An allowlist of tools.** 代理被允许执行的操作——发布评论、投票、置顶、锁定、标记为垃圾、封禁用户、通过私信警告、授予徽章、发送电子邮件、保存和搜索共享记忆。完整列表请参见 [Allowed Tool Calls Overview](#tools-overview)。

当触发条件被触发时，代理会收到一条上下文消息，描述发生的情况（评论、页面，可选的线程/用户/页面上下文），并使用其初始提示和您的社区指南进行提示。随后它调用工具执行操作，并在每次调用时记录理由和置信度分数。

### Agents run asynchronously

代理 **永不阻塞触发它们的用户操作**。读者发布评论，评论被保存并显示在线程中，响应返回后，*此时* 代理才会对其进行处理——可以立即执行，也可以在配置的延迟后执行（参见 [Deferred Triggers](#trigger-deferred-delay)）。代理的任何操作都不会给面向用户的体验增加延迟。

### Why use them

- **Moderate at scale.** 标记明显的垃圾信息并封禁重复违规者，无需全天候监控队列。
- **Welcome new commenters.** 以您的口吻回复首次评论的用户。
- **Surface the best content.** 当实质性顶层评论达到投票阈值时将其置顶。
- **Enforce your guidelines consistently.** 对每条边缘评论使用相同的政策文本。
- **Summarize long threads.** 发布多页辩论的中立摘要。

### What keeps you in control

- **Dry-run mode.** 每个新代理默认以 **Dry Run** 模式发布：它处理触发条件，运行模型，并记录它 *本会* 执行的操作，但不采取真实行动。参见 [Dry-Run Mode](#dry-run-mode)。
- **Approvals.** 任意子集的操作都可以设置为需要人工批准。参见 [Approval Workflow](#approval-workflow)。
- **Per-agent and per-account budgets.** 严格的每日和每月上限。参见 [Budgets Overview](#budgets-overview)。
- **Tool allowlist.** 被禁止的工具会从模型的调色板中剔除——代理根本无法请求这些工具。参见 [Allowed Tool Calls Overview](#tools-overview)。
- **Audit fields on every action.** 模型必须包含理由和置信度分数。这两项会出现在运行时间线和每次批准中。参见 [Run Detail View](#run-detail-view)。
- **EU DSA Article 17.** 在欧盟地区，完全自动化的封禁被阻止。参见 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。
- **No training on your data.** FastComments 使用的提供商不会在您的提示或评论上进行训练。

### Where they fit alongside human moderation

代理和人工版主共享同一评论平台：代理通过相同的渠道（批准、垃圾、封禁、徽章、置顶、锁定、写入）执行操作，这些操作会出现在相同的 [Comment Logs](/guide-moderation.html#comment-logs)、相同的 [Moderation Page](/guide-moderation.html#moderate-comments-page) 和相同的通知流中。代理和人工版主可以看到彼此的工作，并可以相互响应——版主操作本身也是有效的代理触发条件（参见 [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) 等）。

如果您也需要人工参与队列，FastComments 提供 [Human Moderation Services](/guide-moderation.html#human-moderation-services)：我们的版主会与您的代理一起工作，出现在您的仪表板中，遵循您的指南。

---