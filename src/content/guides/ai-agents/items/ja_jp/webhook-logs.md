Each agent webhook has its own delivery log. Reachable from the [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via the **Logs** button on each webhook row.

### ページにあるもの

A paginated table with one row per delivery attempt:

| Column | Meaning |
|---|---|
| When | 試行が発生した時刻。 |
| Event | イベントの種類（`trigger.succeeded`、`approval.requested`など）。 |
| Status | 配信ステータス。 |
| StatusCode | 利用可能な場合にエンドポイントが返したHTTPステータスコード。 |
| Description | 結果の短い説明。HTTPレスポンスが得られなかった失敗配信の場合、基底のエラーメッセージは `{error: <message>}` として格納されます。 |

The page supports pagination only - there are no status, event-type, or date-range filters.

### ログからできること

- **Decide whether a status code should be in No-retry.** If you see your endpoint returning the same `4xx` over and over, that is a strong signal you want to add it to **No-retry status codes** so the platform stops retrying.

### 失敗情報

When a delivery fails without an HTTP response (DNS failure, connection refused, timeout, TLS error, etc.), the raw error message is recorded as `{error: <message>}`. The platform does not categorize these into named buckets like `TIMEOUT` or `DNS_ERROR` - read the error message directly to see what happened.

For HTTP responses, the StatusCode column shows the code returned by your endpoint. Common cases:

- **All attempts: `401` or `403`** - your endpoint is rejecting the signature. Check that you are computing the HMAC over `${timestamp}.${body}` and using the right secret. See [Webhook Signing](#webhook-signing).
- **All attempts: `422`** - your endpoint thinks the payload is invalid. Either fix your endpoint, or add `422` to **No-retry status codes** if the rejection is expected for some events.

### 配信ごとのコンテキスト

Each log entry carries:

- `webhookId` - which webhook config produced this delivery.
- `agentId` - which agent the delivery is about.
- `triggerId` or `approvalId` - the underlying record.
- `domain` - the matched domain.

You can use these to correlate a failed delivery with the run it relates to in [Run History](#run-history).

### 保持期間

`AgentSyncLog` entries have a flat 1-year TTL on `createdAt` regardless of outcome - successful and failed deliveries are retained for the same length of time. Beyond retention, the log entry is gone.

If you need long-term audit, the sustainable pattern is: have the **endpoint itself** persist the deliveries it receives. That decouples your audit log from the platform's retention policy.

### テスト送信

The webhook config form's **Test send** button writes a fake delivery into the same log table so you can verify connectivity end-to-end before relying on real events. Test deliveries are clearly labeled in the log so they do not pollute production failure stats.

### 参照

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) for the retry semantics that drive these logs.
- [Webhook Signing](#webhook-signing) for how to verify on your endpoint.