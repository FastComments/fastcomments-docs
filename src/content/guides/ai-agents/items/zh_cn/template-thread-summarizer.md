**Template ID:** `thread_summarizer`

The Thread Summarizer posts a neutral, single-paragraph summary at the end of a long thread. It uses a 30-minute deferral so the thread can settle before the agent looks at it.

内置提示指示代理不要加入主观看法 —— 这是关键要求。否则模型会倾向使用“在我看来”之类的表述，这在以您的账户显示名发布时读起来很别扭。

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [延迟触发器](#trigger-deferred-delay).

30分钟的延迟意味着代理只会运行一次，在评论发布后半小时对当时线程的状态进行处理。它并不是“对每次回复都做摘要”——延迟触发队列会将同一线程上的多个新评论事件合并，但不会在不同时间窗口中对它们去重。您很可能需要**在提示中添加自定义规则**，例如“如果代理在过去24小时内已对该线程做过摘要，则不要发布新的摘要”，并依赖上下文以及代理的[内存工具](#tools-overview)来强制执行该规则。

### Allowed tools

- [`write_comment`](#tools-overview) - 发布摘要本身。
- [`pin_comment`](#tools-overview) - 将摘要置顶，以便读者在主题顶部看到它。
- [`unpin_comment`](#tools-overview) - 在置顶新摘要之前，取消置顶该代理之前的摘要。

该摘要器无法进行审核或与用户互动。

### Pinning the summary

代理使用 `write_comment` 发布新评论，然后用返回的评论 ID 调用 `pin_comment`。在后续针对同一线程的运行中，提示会指示代理先对其先前的摘要调用 `unpin_comment` —— 平台本身并**不**强制每个线程只有一个置顶评论，所以如果保留先前的摘要为置顶，将会出现并列的两个置顶摘要。在[上下文选项](#context-options)中勾选 "Include parent comment and prior replies in the same thread"，以便代理能够看到之前被置顶的摘要。

### Recommended additions before going live

- **在[上下文选项](#context-options)中勾选 "Include parent comment and prior replies in the same thread"。** 没有线程上下文的摘要器没有意义。
- **调整最小线程大小规则。** 提示默认是“少于 5 条评论”，但在活跃社区中 10–20 条更合适。直接编辑提示。
- **限制到特定的 URL 模式**，如果您只想在长文章页面上进行摘要，而不是在公告或产品页面上。见 [范围：URL 和区域筛选](#scope-url-locale)。
- **注意费用。** 摘要是最耗费 token 的模板，因为它每次运行都会读取整个线程。在启用之前，请在[每日预算](#budgets-overview)中设置严格的上限。

### Avoiding repeat summaries

代理可以使用 [`save_memory`](#tools-overview) 和 [`search_memory`](#tools-overview) —— 您可以扩展提示，指示其记录“summarized {thread urlId}”的笔记，并在再次发布前检查这些笔记。记忆在您租户的所有代理之间共享。

---