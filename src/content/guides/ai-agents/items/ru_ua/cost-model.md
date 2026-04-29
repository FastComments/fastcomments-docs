Agent cost is **token-based**. Every LLM call returns a token count, the platform converts that to USD cents using the model's per-token rate, and the cents are billed against the agent's and tenant's budgets.

### What's billed

- **All LLM calls**, including the call that produces zero tool actions ("the agent decided to do nothing"). Inference is paid even when no action results.
- **Dry-run calls**. Dry-run is "do not act, but still call the LLM" - the LLM call costs the same. See [Режим сухого прогона](#dry-run-mode).
- **Replay calls**. Replays are dry-run runs against historical comments. They cost tokens. See [Тестовые прогоны (воспроизведение)](#test-runs-replays).

### What's not billed

- **Triggers that never produce an LLM call.** Dropped-before-LLM cases (over budget, rate limited, scope mismatch, billing invalid, loop prevention) cost zero tokens. See [Причины отбрасывания](#drop-reasons).
- **Tool dispatch.** Calling `pin_comment` or any other tool does not itself cost tokens - only the LLM round-trip does.
- **`search_memory`.** It is read-only and does not produce its own LLM round-trip.

### Cost per run

A single agent run can call the LLM multiple times - each tool call result is fed back into the model so it can either call another tool or finish. So `tokensUsed` on a run is the sum across all LLM round-trips in that run.

The biggest contributors to per-run token cost:

- **Long [начальные подсказки](#personality-prompt) and [руководства сообщества](#community-guidelines)** - they go in on every run.
- **[Параметры контекста](#context-options)** - thread context, user history, page metadata. Each adds tokens.
- **The comment text itself** - long comments cost more.
- **Multiple tool calls in one run** - each tool's result message is sent back to the model.
- **Memory reads** - `search_memory` returns up to 25 records (capped at 8000 chars total content). Most of those bytes go into the next prompt.

**Max Tokens Per Trigger** (default 20,000) caps the **response** size per LLM call. It does not cap the input size.

### Token-to-cents conversion

The platform applies a single per-tenant-package rate (`flexLLMCostCents` per `flexLLMUnit` tokens). Cost-per-token is package-level, not per-model - both available models ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) bill at the same rate on a given package. The [Просмотр деталей прогона](#run-detail-view) shows the per-run cost in your currency once a run completes.

### Where cost is recorded

Each run records its raw token count and per-run cost. Daily and monthly totals roll up into the [Страница аналитики](#analytics-page).

### How to read cost

- **Per-run cost**: [Просмотр деталей прогона](#run-detail-view) -> `Cost` field.
- **Daily / monthly aggregate**: [Страница аналитики](#analytics-page) -> Budget usage and Daily cost charts.
- **Per-action cost**: also on Run Detail View, useful for tuning when an agent's tool-loop is unusually long.

### See also

- [Choosing a Model](#choosing-a-model) - the bigger lever on cost.
- [Параметры контекста](#context-options) - where added cost comes from.
- [Budgets Overview](#budgets-overview) - hard caps that prevent runaway cost.