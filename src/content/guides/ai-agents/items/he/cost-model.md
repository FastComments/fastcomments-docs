Agent cost is **מבוססת על טוקנים**. Every LLM call returns a token count, the platform converts that to USD cents using the model's per-token rate, and the cents are billed against the agent's and tenant's budgets.

### מה מחויב

- **כל קריאות LLM**, כולל הקריאה שמייצרת אפס פעולות כלי ("הסוכן החליט לא לעשות דבר"). Inference is paid even when no action results.
- **קריאות Dry-run**. Dry-run הוא "אל תפעול, אך עדיין קרא ל-LLM" - the LLM call costs the same. See [Dry-Run Mode](#dry-run-mode).
- **קריאות Replay**. Replays are dry-run runs against historical comments. They cost tokens. See [Test Runs (Replays)](#test-runs-replays).

### מה לא מחויב

- **טריגרים שלעולם לא מייצרים קריאת LLM**. Dropped-before-LLM cases (over budget, rate limited, scope mismatch, billing invalid, loop prevention) cost zero tokens. See [Drop Reasons](#drop-reasons).
- **הפצת כלי**. Calling `pin_comment` or any other tool does not itself cost tokens - only the LLM round-trip does.
- **`search_memory`**. It is read-only and does not produce its own LLM round-trip.

### עלות לכל ריצה

A single agent run can call the LLM multiple times - each tool call result is fed back into the model so it can either call another tool or finish. So `tokensUsed` on a run is the sum across all LLM round-trips in that run.

הגורמים הגדולים ביותר לעלות הטוקנים לכל ריצה:

- **[initial prompts](#personality-prompt) ארוכים** ו**[community guidelines](#community-guidelines)** - they go in on every run.
- **[Context options](#context-options)** - thread context, user history, page metadata. Each adds tokens.
- **טקסט ההערה עצמו** - long comments cost more.
- **קריאות מרובות לכלי בריצה אחת** - each tool's result message is sent back to the model.
- **קריאות זיכרון** - `search_memory` returns up to 25 records (capped at 8000 chars total content). Most of those bytes go into the next prompt.

**Max Tokens Per Trigger** (default 20,000) caps the **response** size per LLM call. It does not cap the input size.

### המרת טוקן לסנטים

The platform applies a single per-tenant-package rate (`flexLLMCostCents` per `flexLLMUnit` tokens). Cost-per-token is package-level, not per-model - both available models ([GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)) bill at the same rate on a given package. The [Run Detail View](#run-detail-view) shows the per-run cost in your currency once a run completes.

### היכן מתועדת העלות

Each run records its raw token count and per-run cost. Daily and monthly totals roll up into the [Analytics page](#analytics-page).

### איך לקרוא את העלות

- **עלות לכל ריצה**: [Run Detail View](#run-detail-view) -> `Cost` field.
- **סיכום יומי / חודשי**: [Analytics page](#analytics-page) -> Budget usage and Daily cost charts.
- **עלות לכל פעולה**: also on Run Detail View, useful for tuning when an agent's tool-loop is unusually long.

### ראה גם

- [Choosing a Model](#choosing-a-model) - the bigger lever on cost.
- [Context Options](#context-options) - where added cost comes from.
- [Budgets Overview](#budgets-overview) - hard caps that prevent runaway cost.

---