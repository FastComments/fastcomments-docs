Each agent runs against one of two LLM models, picked on the **Model** section of the edit form.

### The two options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - the default. Higher reasoning quality, somewhat slower per call. Recommended for moderation-style agents (`Moderator` template, anything that calls `ban_user` or `mark_comment_spam`) where the cost of a wrong call is high.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - faster per call, lower latency. Recommended for high-volume, low-stakes agents (welcome greeter, thread pinner) where you want responses within seconds and the consequences of an off call are minor.

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
