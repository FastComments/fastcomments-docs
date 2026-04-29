每个代理在编辑表单的 **Model** 部分选择的两种 LLM 模型之一上运行。

### 两个选项

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - 默认。推理质量更高，每次调用稍慢一些。推荐用于审核类代理（`Moderator` 模板，或任何调用 `ban_user` 或 `mark_comment_spam` 的场景），在错误调用代价很高时使用。

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - 每次调用更快，延迟更低。推荐用于高流量、低风险的代理（欢迎接待、置顶主题等），需要在几秒内得到响应且错误调用后果不大。

两个模型都支持函数调用，都通过相同的 OpenAI 兼容 API 运行，并共享相同的每个工具的 schemas —— 因此你可以随时在它们之间切换已保存的代理，而无需其它配置更改。

### 成本差异

两种模型的每 token 成本不同。代理的 [budget caps](#budgets-overview) 以你的账户货币计价，而不是以 tokens 计价，因此从一种模型切换到另一种会改变在你的日/月上限内能运行多少次。一次运行完成后，[Run History](#run-history) 页面会显示以你的货币计价的每次运行成本 — 在切换后观察几次运行是评估新消耗速率的最简单方法。

### 每次运行的 tokens

模型的响应 token 使用量在每次触发时以 **tokensUsed** 记录。它包含在 `trigger.succeeded` 和 `trigger.failed` 的 webhook payload 中（参见 [Webhook Payloads](#webhook-payloads)），并显示在 [Run Detail View](#run-detail-view)。数量取决于：

- 你包含了多少 [Context](#context-options) —— 线程上下文、用户历史和页面元数据都会增加 token 数量。
- 你的 [Initial prompt](#personality-prompt) 和 [Community guidelines](#community-guidelines) 有多长。
- 代理在单次运行中调用了多少工具（每次工具调用及其结果都会绕回模型进行往返）。

**Max Tokens Per Trigger**（默认 20,000）是每次运行的上限，可为每个代理设置。

### 切换模型

你可以随时在编辑表单中切换模型。已有的运行历史和分析会保留它们原始的 token 和成本数据 —— 这些数据是在运行时记录的。新模型只适用于你保存后开始的运行。

没有 "use whichever model is cheaper" 这个选项。选择是针对每个代理明确指定的。