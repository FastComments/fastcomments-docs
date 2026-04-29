FastComments 提供四个入门模板，免去你从头编写可运行代理的麻烦。可在 [AI Agents 页面](https://fastcomments.com/auth/my-account/ai-agents) 通过点击 **浏览模板** 访问它们。

当你选择一个模板时：

1. 代理将以 **Status: Dry Run** 创建，并使用基于模板的内部名称（`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`）。如果该名称已被你的租户占用，则会追加一个数字后缀。
2. 你会直接进入编辑表单，所有内容均已预填 - prompt、触发器、允许的操作以及任何阈值。顶部横幅显示 "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. 目前尚未启用任何功能。代理在你保存并且要么保持 dry-run（用于观察），要么切换为 Enabled 之前，不会执行任何操作。

### 四个模板

- **[Moderator](#template-moderator)** - 审查新评论和被标记的评论，对首次违规者发出警告，只有在警告后才升级为禁令。触发于新评论和标记阈值达到时（默认标记阈值: 3）。允许的工具: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 用简短且个性化的欢迎语热情回复首次评论者。触发于 new-user-first-comment。允许的工具: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 当优质顶层评论达到投票阈值（默认: 10）时将其置顶，先取消之前置顶的评论。触发于投票阈值达到。允许的工具: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 在长话题中延迟一段时间后发布中立的单段摘要，然后将其置顶。触发于新评论，延迟 30 分钟以便话题在摘要前稳定。允许的工具: `write_comment`, `pin_comment`, `unpin_comment`.

### 自定义模板

模板只是起点，而非约束。你应该：

- 调整 **Initial prompt** 以匹配你社区的语气。
- 添加或移除 **Triggers** 以符合代理应运行的频率。
- 为任何敏感操作添加 **Approvals** —— 我们强烈建议在版主类模板中将 `ban_user` 设置为需审批后才能执行。
- 添加 **Community guidelines**，以便代理始终如一地应用你写明的政策。参见 [Community Guidelines](#community-guidelines).
- 为每个代理设置与预期触发次数相匹配的 **Budgets**。

模板只是一个预填合理默认值的载体；保存后，代理即归你所有。