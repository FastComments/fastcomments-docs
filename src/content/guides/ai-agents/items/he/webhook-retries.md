Agent webhooks retry on failure. Delivery is **שלח ושכח מנקודת המבט של הסוכן** - משלוח נכשל לא חוסם את ביצועי הסוכן או מבטל שום פעולה - ותור + cron מנהלים ניסיונות חוזרים באופן אסינכרוני.

### Queue model

Each event is queued **once per matching webhook**. So if you have three webhooks subscribed to `trigger.succeeded` for a given agent + domain, the platform queues three deliveries; each is delivered and retried independently. A failure on one webhook never affects the others.

### What's retried

A delivery is retried when:

- The HTTP request **does not complete** (כשל ב-DNS, החיבור נדחה, פסק-זמן).
- The HTTP response code is any non-`2xx` status that is not in the configured **No-retry status codes** list.

A delivery is **not retried** when:

- The response code is `2xx` (success).
- The response code is in the configured **No-retry status codes** list. By default this list is empty - any non-`2xx` is retried.

### Configuring no-retry codes

The webhook config form has a **No-retry status codes** field (multi-value). Common entries:

- `410` - Gone. נקודת הקצה הועברה לצמיתות או שהמשאב נעלם. ניסיון חוזר מבזבז רוחב פס משני הצדדים.
- `422` - Unprocessable Entity. נקודת הקצה הבינה את המטען אך חשבה שהוא לא תקין. ניסיון חוזר עם אותו מטען יחזיר את אותה תשובה.
- `400` - Bad Request, באותו ההיגיון.

Adding a code here means: when the endpoint returns it, mark the delivery as failed-terminal and stop retrying.

### Retry schedule

A background worker runs every few seconds and processes any deliveries whose next attempt time has passed.

After each failure, the next-attempt time is pushed forward with **linear backoff**: the wait grows as `60 seconds * attempt count` (so attempt 1 waits 1 minute, attempt 2 waits 2 minutes, and so on).

After 99 failed attempts (or 3 in local development), the delivery is given up and dropped from the queue. The delivery log entries do persist and remain visible in the [יומני משלוחי Webhook](#webhook-logs) page until they expire.

### Idempotence on your side

Because we retry, your endpoint **must be idempotent**. The same `triggerId` (or `approvalId`) can arrive more than once. Your endpoint should:

- Use a unique key (`triggerId` for trigger events, `approvalId` for approval events) as a dedup token.
- Accept duplicate deliveries gracefully (return `200` the second time).

A non-idempotent endpoint will eventually double-process some deliveries, especially during transient outages where one timeout retries 30 seconds later but the original request actually succeeded.

### Ordering

Deliveries are **not strictly ordered**. A `trigger.succeeded` and a downstream `approval.requested` (from the same run) can arrive in either order if one retries and the other does not. Your endpoint should not assume causal ordering.

If you need ordering, use the timestamps - `occurredAt` on the envelope, plus the trigger/approval `createdAt` in the data block - to reconstruct order on your side.

### Cleanup

Deliveries are removed from the queue as soon as they either succeed or hit the attempt cap. The platform does not retain terminally-failed deliveries in the queue itself; the durable record of each attempt lives in the [יומני משלוחי Webhook](#webhook-logs) page.

### Where to look when retries fail

The [יומני משלוחי Webhook](#webhook-logs) page is the place to see why a webhook is failing. Common causes:

- **כשל בפתרון DNS** - ה-URL שגוי או הדומיין נעלם.
- **שגיאות TLS** - התעודה של נקודת הקצה אינה תקפה או פגה.
- **החיבור נדחה / פסק-זמן** - נקודת הקצה לא פעילה.
- **תגובות 5xx** - נקודת הקצה פעילה אך אירעה שגיאה. גוף התגובה (מקוצר) נרשם.
- **תגובות 4xx** - נקודת הקצה דחתה את המטען. אם זה מכוון, הוסף את הקוד ל-**No-retry status codes**.

### Pause an unhealthy webhook

If a webhook is consistently failing, the cleanest fix is to delete it (or temporarily clear its event subscription list). The platform does not auto-disable failing webhooks - they keep retrying until the delivery is given up.