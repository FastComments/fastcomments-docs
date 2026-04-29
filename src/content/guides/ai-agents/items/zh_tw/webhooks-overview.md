Agent webhooks 是平台在 agent 執行完成或審核狀態改變時觸發的 HTTP 回調。使用它們可以將 agent 的活動轉送到您自己的系統——審核儀表板、稽核日誌、Slack 頻道、升級工具。

Configured under the **Webhooks** tab on the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### What gets delivered

Four event types:

- **`trigger.succeeded`** - 一次 agent 執行成功完成。
- **`trigger.failed`** - 一次 agent 執行發生錯誤。
- **`approval.requested`** - 一個操作被佇列，等待人工審核。
- **`approval.decided`** - 一次審核被核准、拒絕，或執行時失敗。

See [Webhook Events](#webhook-events) for which events fire when, and [Webhook Payloads](#webhook-payloads) for the schema of each.

### What's not delivered

- **Per-tool-action webhooks.** 一次執行中呼叫 `pin_comment` 並不會為 pin 單獨觸發一個 webhook —— 該動作會包含在執行的 `trigger.succeeded` payload 中。若您想要逐動作傳送，請解析 trigger payload 中的 `actions` 陣列。
- **Dropped triggers.** 未被派發的觸發（超出預算、範圍錯誤）不會觸發 webhook。被丟棄的觸發僅能在 [Analytics page](#analytics-page) 查看。
- **Replay-produced triggers.** 測試執行不會觸發 webhook。請參閱 [Test Runs (Replays)](#test-runs-replays)。

### Configuration

For each webhook you set:

- **URL** - 要 POST 的 HTTPS 端點。
- **Domain** - 此 webhook 適用的留言網域（您的租戶可能承載多個網域）。`*` 匹配所有網域；`*.example.com` 是一個 glob；精確網域則只匹配該網域。
- **Events** - 要訂閱的四種事件類型中的哪些。
- **Agents** - 留空表示「所有 agents」，或填入特定 agent ID 的清單作為篩選。
- **Method** - POST 或 PUT（預設為 POST）。
- **No-retry status codes** - 應視為終止失敗、不再重試的 HTTP 回應碼（例如 410 Gone、422 Unprocessable）。參見 [Webhook Retries](#webhook-retries)。

多個 webhook 可以訂閱相同事件 —— 每個 webhook 各自佇列並傳送到其設定的 URL。

### Per-domain matching

A given event is delivered to **every webhook whose `domain` field matches the comment's domain**. The matching uses a simple glob:

- Exact: `example.com` matches only `example.com`.
- Wildcard star: `*` matches every domain.
- Subdomain glob: `*.example.com` matches `blog.example.com`, `forum.example.com`, but not `example.com` itself.

The domain on a payload is the first non-null result from: the comment's `domain`, the URL parsed against your tenant's domain configuration, or the `Page` lookup by `urlId`.

### Per-agent filtering

The **Agents** field lets a webhook subscribe to only certain agents. Empty means "all agents". When non-empty, the webhook only fires for agents in the list.

This is useful when you have one webhook for moderation events and another for engagement events, both routing to different downstream systems.

### Test send

The webhook config UI has a **Test send** button that posts a fake payload to the URL so you can verify connectivity, signing, and your endpoint's response code without waiting for a real event.

### Delivery logs

Every delivery (and every retry) lands in the [Webhook Delivery Logs](#webhook-logs) page so you can see what happened on the wire.

### Signing

Every webhook is signed with HMAC-SHA256 using your tenant's API secret. See [Webhook Signing](#webhook-signing).

### Eligibility

Agent webhooks require valid billing on the tenant. They use the same signing/secret infrastructure as your existing comment webhooks - if you have already integrated comment webhooks (see the [Webhooks guide](/guide-webhooks.html)), the agent webhook integration is the same shape with new event types.