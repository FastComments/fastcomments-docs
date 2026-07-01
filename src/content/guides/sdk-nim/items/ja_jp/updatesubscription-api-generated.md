## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | いいえ |  |
| userId | string = "" | いいえ |  |

## レスポンス

戻り値: [`Option[UpdateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_subscription_api_response.nim)

## 例

[inline-code-attrs-start title = 'updateSubscription の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let subscriptionData = UpdateAPIUserSubscriptionData(
  planId = "premium-plan",
  isActive = true,
  expiresAt = "2025-01-01",
)

let (responseOpt, httpResponse) = client.updateSubscription(
  tenantId = "my-tenant-123",
  id = "sub-456",
  updateAPIUserSubscriptionData = subscriptionData,
  userId = "user-789",
)

if responseOpt.isSome:
  let subscriptionResult = responseOpt.get()
[inline-code-end]

---