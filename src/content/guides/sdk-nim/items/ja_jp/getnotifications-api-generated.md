---
## パラメータ

| 名称 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| urlId | string | はい |  |
| fromCommentId | string | いいえ |  |
| viewed | bool | いいえ |  |
| skip | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## 例

[inline-code-attrs-start title = 'getNotifications の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(tenantId = "my-tenant-123", userId = "user-456", urlId = "news/article-title", fromCommentId = "cmt-789", viewed = false, skip = 0.0)
if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---