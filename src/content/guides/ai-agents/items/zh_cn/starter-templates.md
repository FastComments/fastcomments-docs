FastComments 提供五个入门模板，这样你就无需从零开始编写可用的代理。你可以在[AI Agents 页面](https://fastcomments.com/auth/my-account/ai-agents)通过点击 **浏览模板** 来访问它们。

当你选择一个模板时：

1. 该代理将以 **状态：模拟运行** 创建，并使用基于模板的内部名称（`tos_enforcer`，`welcome_greeter`，`top_comment_pinner`，`thread_summarizer`，`gaslight_detector`）。如果该名称在你的租户中已被占用，则会添加数字后缀。
2. 你将直接来到带有所有预填内容的编辑表单 — 提示、触发器、允许的操作以及任何阈值。顶部横幅显示“已基于 {templateName} 模板创建。请检查下面的设置，准备好后将状态更改为已启用。”
3. 目前尚未启用任何功能。代理在你保存并选择保持模拟运行（以便观察）或切换为已启用之前不会采取任何行动。

### 五个模板

- **[版主](#template-moderator)** - 审查新评论和被举报的评论，先警告首次违规者，只有在警告后才升级为封禁。触发于新评论以及举报阈值达成（默认举报阈值：3）。允许使用的工具：`mark_comment_approved`，`mark_comment_spam`，`warn_user`，`ban_user`。

- **[欢迎问候者](#template-welcome-greeter)** - 对首次评论者以简短、亲切的欢迎回复。触发条件为 new-user-first-comment。允许的工具：`write_comment`。

- **[置顶评论钉住器](#template-top-comment-pinner)** - 一旦实质性顶级评论超过投票阈值（默认：10）就将其置顶，先取消先前置顶的评论。触发条件为投票阈值达成。允许的工具：`pin_comment`，`unpin_comment`。

- **[线程摘要器](#template-thread-summarizer)** - 在延迟后对长讨论串发布中性、单段落的摘要，然后将其置顶。触发条件为新评论，延迟 30 分钟以便讨论串在摘要前稳定。允许的工具：`write_comment`，`pin_comment`，`unpin_comment`。

- **[煤气灯侦测器](#template-gaslight-detector)** - 监视评论编辑，以发现在讨论中间改写内容从而扭曲回复的情况，恢复原始文本并向作者发送私信。触发条件为评论编辑。允许的工具：`edit_comment`，`warn_user`，`send_dm`。

### 定制模板

模板是起点，而不是约束。你应当：

- 调整 **初始提示** 以匹配你社区的语气。
- 添加或移除 **触发器**，以调整代理的运行频率。
- 对任何敏感操作添加 **审批** — 我们强烈建议在版主类型的模板中将 `ban_user` 设为需审批。
- 添加 **社区准则**，以便代理一致地应用你编写的政策。参见 [社区准则](#community-guidelines)。
- 为每个代理设置合适的 **预算**，以配合你预期的触发次数。

模板只是一个预填合理默认值的工具；保存后，该代理即归你所有。

---