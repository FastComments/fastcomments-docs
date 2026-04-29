Hver agent-webhook har sin egen leveringslog. Tilgængelig fra [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via knappen **Logfiler** på hver webhook-række.

### What's on the page

A paginated table with one row per delivery attempt:

| Column | Meaning |
|---|---|
| When | When the attempt happened. |
| Event | The event type (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | The delivery status. |
| StatusCode | HTTP status code returned by your endpoint, when available. |
| Description | A short description of the outcome. For failed deliveries where no HTTP response was received, the underlying error message is stored as `{error: <message>}`. |

The page supports pagination only - there are no status, event-type, or date-range filters.

### What you can do from logs

- **Decide whether a status code should be in No-retry.** If you see your endpoint returning the same `4xx` over and over, that is a strong signal you want to add it to **No-retry status codes** so the platform stops retrying.

### Failure information

When a delivery fails without an HTTP response (DNS failure, connection refused, timeout, TLS error, etc.), the raw error message is recorded as `{error: <message>}`. The platform does not categorize these into named buckets like `TIMEOUT` or `DNS_ERROR` - read the error message directly to see what happened.

For HTTP responses, the StatusCode column shows the code returned by your endpoint. Common cases:

- **All attempts: `401` or `403`** - your endpoint is rejecting the signature. Check that you are computing the HMAC over `${timestamp}.${body}` and using the right secret. See [Webhook Signing](#webhook-signing).
- **All attempts: `422`** - your endpoint thinks the payload is invalid. Either fix your endpoint, or add `422` to **No-retry status codes** if the rejection is expected for some events.

### Per-delivery context

Each log entry carries:

- `webhookId` - which webhook config produced this delivery.
- `agentId` - which agent the delivery is about.
- `triggerId` or `approvalId` - the underlying record.
- `domain` - the matched domain.

You can use these to correlate a failed delivery with the run it relates to in [Run History](#run-history).

### Retention

`AgentSyncLog` entries have a flat 1-year TTL on `createdAt` regardless of outcome - successful and failed deliveries are retained for the same length of time. Beyond retention, the log entry is gone.

If you need long-term audit, the sustainable pattern is: have the **endpoint itself** persist the deliveries it receives. That decouples your audit log from the platform's retention policy.

### Test send

The webhook config form's **Test send** button writes a fake delivery into the same log table so you can verify connectivity end-to-end before relying on real events. Test deliveries are clearly labeled in the log so they do not pollute production failure stats.

### See also

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) for the retry semantics that drive these logs.
- [Webhook Signing](#webhook-signing) for how to verify on your endpoint.