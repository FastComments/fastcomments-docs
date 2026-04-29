Agent webhooks retry on failure. Delivery is **fire-and-forget from the agent's perspective** - a failed delivery does not block agent execution or roll back any actions - and a queue + cron drives retries asynchronously.

### Queue model

Each event is queued **once per matching webhook**. So if you have three webhooks subscribed to `trigger.succeeded` for a given agent + domain, the platform queues three deliveries; each is delivered and retried independently. A failure on one webhook never affects the others.

### What's retried

A delivery is retried when:

- The HTTP request **does not complete** (DNS failure, connection refused, timeout).
- The HTTP response code is any non-2xx status that is not in the configured **No-retry status codes** list.

A delivery is **not retried** when:

- The response code is `2xx` (success).
- The response code is in the configured **No-retry status codes** list. By default this list is empty - any non-2xx is retried.

### Configuring no-retry codes

The webhook config form has a **No-retry status codes** field (multi-value). Common entries:

- `410` - Gone. Your endpoint is permanently moved or the resource is gone. Retrying just wastes both sides' bandwidth.
- `422` - Unprocessable Entity. Your endpoint understood the payload but considered it invalid. Retrying with the same payload will get the same answer.
- `400` - Bad Request, in the same spirit.

Adding a code here means: when the endpoint returns it, mark the delivery as failed-terminal and stop retrying.

### Retry schedule

A background worker runs every few seconds and processes any deliveries whose next attempt time has passed.

After each failure, the next-attempt time is pushed forward with **linear backoff**: the wait grows as `60 seconds * attempt count` (so attempt 1 waits 1 minute, attempt 2 waits 2 minutes, and so on).

After 99 failed attempts (or 3 in local development), the delivery is given up and dropped from the queue. The delivery log entries do persist and remain visible in the [Webhook Delivery Logs](#webhook-logs) page until they expire.

### Idempotence on your side

Because we retry, your endpoint **must be idempotent**. The same `triggerId` (or `approvalId`) can arrive more than once. Your endpoint should:

- Use a unique key (`triggerId` for trigger events, `approvalId` for approval events) as a dedup token.
- Accept duplicate deliveries gracefully (return 200 the second time).

A non-idempotent endpoint will eventually double-process some deliveries, especially during transient outages where one timeout retries 30 seconds later but the original request actually succeeded.

### Ordering

Deliveries are **not strictly ordered**. A `trigger.succeeded` and a downstream `approval.requested` (from the same run) can arrive in either order if one retries and the other does not. Your endpoint should not assume causal ordering.

If you need ordering, use the timestamps - `occurredAt` on the envelope, plus the trigger/approval `createdAt` in the data block - to reconstruct order on your side.

### Cleanup

Deliveries are removed from the queue as soon as they either succeed or hit the attempt cap. The platform does not retain terminally-failed deliveries in the queue itself; the durable record of each attempt lives in the [Webhook Delivery Logs](#webhook-logs) page.

### Where to look when retries fail

The [Webhook Delivery Logs](#webhook-logs) page is the place to see why a webhook is failing. Common causes:

- **DNS resolution failure** - the URL is wrong or the domain is gone.
- **TLS errors** - your endpoint's certificate is invalid or expired.
- **Connection refused / timeout** - your endpoint is down.
- **5xx responses** - your endpoint is up but errored. The response body (truncated) is recorded.
- **4xx responses** - your endpoint rejected the payload. If this is intentional, add the code to **No-retry status codes**.

### Pause an unhealthy webhook

If a webhook is consistently failing, the cleanest fix is to delete it (or temporarily clear its event subscription list). The platform does not auto-disable failing webhooks - they keep retrying until the delivery is given up.
