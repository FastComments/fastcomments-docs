Each agent runs against one of two LLM models, picked on the **Model** section of the edit form.

### The two options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - デフォルト。推論品質が高く、呼び出しごとにやや遅い。誤った呼び出しのコストが高いモデレーション系エージェント（`Moderator` template、`ban_user` や `mark_comment_spam` を呼ぶようなもの）に推奨されます。

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - 呼び出しごとに高速でレイテンシが低い。数秒以内の応答が必要で、誤動作の影響が小さい大量処理向けのエージェント（ウェルカム挨拶、スレッドピンなど）に推奨されます。

Both models support function calling, both run via the same OpenAI-compatible API, and both share the same per-tool schemas - so you can switch a saved agent between them at any time without other config changes.

### Cost differences

The two models have different per-token costs. The agent's [予算上限](#budgets-overview) are denominated in your account currency, not in tokens, so a switch from one model to the other changes how many runs fit inside your daily and monthly caps. The [実行履歴](#run-history) page shows the per-run cost in your currency once a run completes - watching a few runs after a switch is the easiest way to gauge the new burn rate.

### Tokens per run

The model's response token usage is logged on every trigger as **tokensUsed**. It is included on the `trigger.succeeded` and `trigger.failed` webhook payloads (see [Webhook ペイロード](#webhook-payloads)) and shown in [実行詳細ビュー](#run-detail-view). The amount depends on:

- How much [コンテキスト](#context-options) you include - thread context, user history, and page metadata all add tokens.
- How long your [初期プロンプト](#personality-prompt) and [コミュニティガイドライン](#community-guidelines) are.
- How many tools the agent calls in a single run (each tool call and its result round-trips through the model).

**Max Tokens Per Trigger** (default 20,000) is the upper bound per run, set per-agent.

### Switching models

You can switch models on the edit form at any time. Existing run history and analytics keep their original token and cost numbers - they are recorded at run time. The new model only applies to runs that start after you save.

There is no "use whichever model is cheaper" option. The choice is explicit per agent.