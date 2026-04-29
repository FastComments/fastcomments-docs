**Template ID:** `thread_summarizer`

The Thread Summarizer posts a neutral, single-paragraph summary at the end of a long thread. It uses a 30-minute deferral so the thread can settle before the agent looks at it.

The built-in prompt instructs the agent not to editorialize - this is load-bearing. Without it the model gravitates to "in my view" framing that reads badly under your account's display name.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Deferred Triggers](#trigger-deferred-delay).

The 30-minute delay means the agent runs once, half an hour after a comment lands, against whatever the thread looks like at that moment. It is not "summarize on every reply" - the deferred-trigger queue coalesces multiple new-comment events on the same thread, but does not de-duplicate them across separate windows. You will likely want to **add a custom rule in your prompt** like "do not post a new summary if the agent has already summarized this thread in the last 24 hours" and rely on context plus the agent's [memory tools](#tools-overview) to enforce it.

### Allowed tools

- [`write_comment`](#tools-overview) - posts the summary itself.
- [`pin_comment`](#tools-overview) - pins the summary so readers see it at the top of the thread.
- [`unpin_comment`](#tools-overview) - unpins a prior summary by the same agent before pinning the new one.

The summarizer cannot moderate or interact with users.

### Pinning the summary

The agent posts a new comment with `write_comment`, then calls `pin_comment` with the returned comment ID. On subsequent runs against the same thread, the prompt instructs it to call `unpin_comment` on its prior summary first - the platform itself does **not** enforce a single-pinned-comment rule per thread, so leaving the previous summary pinned will result in two pinned summaries side by side. Tick "Include parent comment and prior replies in the same thread" in [Context Options](#context-options) so the agent can see the prior pinned summary.

### Recommended additions before going live

- **Tick "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). A summarizer with no thread context is useless.
- **Tune the minimum-thread-size rule.** "Fewer than 5 comments" is the prompt's default, but in busy communities 10-20 is more appropriate. Edit the prompt directly.
- **Restrict to specific URL patterns** if you only want summaries on long-form pages, not announcements or product pages. See [Scope: URL and Locale Filters](#scope-url-locale).
- **Watch costs.** Summarization is the most token-heavy template because it reads the whole thread on every run. Set a tight [daily budget](#budgets-overview) before flipping to Enabled.

### Avoiding repeat summaries

The agent has access to [`save_memory`](#tools-overview) and [`search_memory`](#tools-overview) - you can extend the prompt to instruct it to record "summarized {thread urlId}" notes and check for them before posting again. Memory is shared across all agents in your tenant.
