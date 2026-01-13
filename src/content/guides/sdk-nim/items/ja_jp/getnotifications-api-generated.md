## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| urlId | string | はい |  |
| fromCommentId | string | いいえ |  |
| viewed | bool | いいえ |  |
| skip | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications200response.nim)

## 例

[inline-code-attrs-start title = 'getNotifications の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(
  tenantId = "fastcomments-tenant-42",
  userId = "",
  urlId = "news/latest-tech-innovations",
  fromCommentId = "",
  viewed = false,
  skip = 0.0
)

if response.isSome:
  let notifications = response.get()
  echo "Received notifications: ", notifications
else:
  echo "No notifications, response: ", httpResponse
[inline-code-end]

---