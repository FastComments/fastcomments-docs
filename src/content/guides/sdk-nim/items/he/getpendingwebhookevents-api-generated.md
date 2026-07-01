---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetPendingWebhookEventsOptions | No |  |

## תגובה

מחזיר: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  options = GetPendingWebhookEventsOptions()
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]

---