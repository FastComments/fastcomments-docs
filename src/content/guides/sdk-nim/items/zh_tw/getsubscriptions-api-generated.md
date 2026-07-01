---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string = "" | 否 |  |

## 回應

Returns: [`Option[GetSubscriptionsAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_subscriptions_api_response.nim)

## 範例

[inline-code-attrs-start title = 'getSubscriptions 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (subscriptionsOpt, httpResp) = client.getSubscriptions(tenantId = "my-tenant-123", userId = "user-456")
if subscriptionsOpt.isSome:
  let subscriptions = subscriptionsOpt.get()
  echo subscriptions
[inline-code-end]

---