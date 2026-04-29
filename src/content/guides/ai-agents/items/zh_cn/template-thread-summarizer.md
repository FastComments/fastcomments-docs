---
**Template ID:** `thread_summarizer`

线程摘要器会在长帖的末尾发布一个中立的单段摘要。它使用 30 分钟的延迟，以便在代理查看之前让讨论线程稳定下来。

### Built-in initial prompt

[inline-code-attrs-start title = '线程摘要模板初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

“不要加入主观评论”这一指示是关键的。没有它，模型会倾向于使用“在我看来”的表述，这在以你的账户显示名称发布时读起来效果不好。

### Triggers

- **New comment posted** (`COMMENT_ADD`)。
- **Trigger delay**：30 分钟（1800 秒）。参见 [Deferred Triggers](#trigger-deferred-delay)。

30 分钟的延迟意味着代理会在评论发布后半小时运行一次，基于届时该线程的状态进行操作。它并不是“在每条回复时都进行摘要”——延迟触发队列会将同一线程上的多个新评论事件合并，但不会跨不同时间窗去重。你很可能想要在提示中**添加自定义规则**，例如“如果代理在过去 24 小时内已对该线程进行过摘要，则不要发布新的摘要”，并依赖上下文以及代理的 [memory tools](#tools-overview) 来执行该规则。

### Allowed tools

- [`write_comment`](#tools-overview) - 发布摘要本身。
- [`pin_comment`](#tools-overview) - 将摘要置顶，以便读者在该线程顶部看到它。
- [`unpin_comment`](#tools-overview) - 在置顶新摘要之前，取消置顶同一代理先前的摘要。

摘要器不能对用户进行管理或互动。

### Pinning the summary

代理使用 `write_comment` 发布一条新评论，然后使用返回的评论 ID 调用 `pin_comment`。在随后针对同一线程的运行中，提示指示代理先对其先前的摘要调用 `unpin_comment` —— 平台本身并不强制每个线程只有一条置顶评论，因此如果保留先前的摘要为置顶状态，将会导致并列出现两条置顶摘要。请在 [Context Options](#context-options) 中勾选 “Include parent comment and prior replies in the same thread”，以便代理可以看到先前置顶的摘要。

### Recommended additions before going live

- **在 [Context Options](#context-options) 中勾选 “Include parent comment and prior replies in the same thread”。** 没有线程上下文的摘要器是无用的。
- **调优最小线程大小规则。** “少于 5 条评论”是提示的默认值，但在活跃社区中 10–20 条更为合适。直接编辑提示。
- **限制到特定 URL 模式**，如果你只希望在长篇页面上生成摘要，而不是在公告或产品页面上。参见 [Scope: URL and Locale Filters](#scope-url-locale)。
- **注意成本。** 摘要是令牌消耗最多的模板，因为它每次运行都会读取整个线程。在启用之前为其设置严格的 [daily budget](#budgets-overview)。

### Avoiding repeat summaries

代理可以使用 [`save_memory`](#tools-overview) 和 [`search_memory`](#tools-overview) —— 你可以扩展提示，指示其记录 “summarized {thread urlId}” 的笔记，并在再次发布前进行检查。内存在线下租户的所有代理之间共享。

---