Agent webhooks are HTTP callbacks fired by the platform when an agent run completes or an approval changes state. Use them to forward agent activity into your own systems - moderation dashboards, audit logs, Slack channels, escalation tools.

Configured under the **Webhooks** tab on the [AI エージェントページ](https://fastcomments.com/auth/my-account/ai-agents).

### 配信されるもの

Four event types:

- **`trigger.succeeded`** - an agent run completed successfully.
- **`trigger.failed`** - an agent run errored.
- **`approval.requested`** - an action was queued for human approval.
- **`approval.decided`** - an approval was approved, rejected, or execution-failed.

See [Webhook イベント](#webhook-events) for which events fire when, and [Webhook ペイロード](#webhook-payloads) for the schema of each.

### 配信されないもの

- **Per-tool-action webhooks.** A run that calls `pin_comment` does not fire a separate webhook for the pin - the action is included in the run's `trigger.succeeded` payload. If you want per-action delivery, parse the `actions` array on the trigger payload.
- **Dropped triggers.** A trigger that does not dispatch (over budget, wrong scope) does not fire a webhook. Drops are visible only in the [Analytics ページ](#analytics-page).
- **Replay-produced triggers.** Test runs do not fire webhooks. See [テスト実行（リプレイ）](#test-runs-replays).

### 設定

For each webhook you set:

- **URL** - the HTTPS endpoint to POST to.
- **Domain** - the comment domain this webhook applies to (your tenant may host multiple domains). `*` matches all domains; `*.example.com` is a glob; an exact domain matches only that one.
- **Events** - which of the four event types to subscribe to.
- **Agents** - empty for "all agents", or a list of specific agent IDs to filter.
- **Method** - POST or PUT (default POST).
- **No-retry status codes** - HTTP response codes that should be treated as terminal failures, not retried (e.g., 410 Gone, 422 Unprocessable). See [Webhook Retries](#webhook-retries).

Multiple webhooks can subscribe to the same event - each one queues independently and is delivered to its own URL.

### ドメインごとのマッチング

A given event is delivered to **every webhook whose `domain` field matches the comment's domain**. The matching uses a simple glob:

- Exact: `example.com` matches only `example.com`.
- Wildcard star: `*` matches every domain.
- Subdomain glob: `*.example.com` matches `blog.example.com`, `forum.example.com`, but not `example.com` itself.

The domain on a payload is the first non-null result from: the comment's `domain`, the URL parsed against your tenant's domain configuration, or the `Page` lookup by `urlId`.

### エージェントごとのフィルタリング

The **Agents** field lets a webhook subscribe to only certain agents. Empty means "all agents". When non-empty, the webhook only fires for agents in the list.

This is useful when you have one webhook for moderation events and another for engagement events, both routing to different downstream systems.

### テスト送信

The webhook config UI has a **Test send** button that posts a fake payload to the URL so you can verify connectivity, signing, and your endpoint's response code without waiting for a real event.

### 配信ログ

Every delivery (and every retry) lands in the [Webhook 配信ログ](#webhook-logs) page so you can see what happened on the wire.

### 署名

Every webhook is signed with HMAC-SHA256 using your tenant's API secret. See [Webhook 署名](#webhook-signing).

### 適格性

Agent webhooks require valid billing on the tenant. They use the same signing/secret infrastructure as your existing comment webhooks - if you have already integrated comment webhooks (see the [Webhooks ガイド](/guide-webhooks.html)), the agent webhook integration is the same shape with new event types.