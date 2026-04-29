Each agent runs against one of two LLM models, picked on the **Model** section of the edit form.

### The two options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - 預設。推理品質較高，但每次呼叫稍慢。建議用於審核型代理（`Moderator` template、任何會呼叫 `ban_user` 或 `mark_comment_spam` 的情境），當誤判的代價很高時使用。

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - 每次呼叫較快、延遲較低。建議用於高流量、低風險的代理（歡迎致詞、討論串置頂者），當你希望在數秒內得到回應且錯誤呼叫的後果較小時使用。

Both models support function calling, both run via the same OpenAI-compatible API, and both share the same per-tool schemas - so you can switch a saved agent between them at any time without other config changes.

### Cost differences

The two models have different per-token costs. The agent's [budget caps](#budgets-overview) are denominated in your account currency, not in tokens, so a switch from one model to the other changes how many runs fit inside your daily and monthly caps. The [Run History](#run-history) page shows the per-run cost in your currency once a run completes - watching a few runs after a switch is the easiest way to gauge the new burn rate.

### Tokens per run

The model's response token usage is logged on every trigger as **tokensUsed**. It is included on the `trigger.succeeded` and `trigger.failed` webhook payloads (see [Webhook Payloads](#webhook-payloads)) and shown in [Run Detail View](#run-detail-view). The amount depends on:

- How much [Context](#context-options) you include - thread context, user history, and page metadata all add tokens.
- How long your [Initial prompt](#personality-prompt) and [Community guidelines](#community-guidelines) are.
- How many tools the agent calls in a single run (each tool call and its result round-trips through the model).

**Max Tokens Per Trigger** (default 20,000) is the upper bound per run, set per-agent.

### Switching models

You can switch models on the edit form at any time. Existing run history and analytics keep their original token and cost numbers - they are recorded at run time. The new model only applies to runs that start after you save.

There is no "use whichever model is cheaper" option. The choice is explicit per agent.