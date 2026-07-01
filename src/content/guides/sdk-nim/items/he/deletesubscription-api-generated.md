## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string = "" | No |  |

## תשובה

מחזיר: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.deleteSubscription(
  tenantId = "my-tenant-123",
  id = "sub-789",
  userId = ""
)

if maybeResp.isSome:
  let apiResult = maybeResp.get()
  # השתמש ב‑apiResult לפי הצורך
[inline-code-end]