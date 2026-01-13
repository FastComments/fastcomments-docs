## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenantId | string | 必須 |  |
| userId | string | 任意 |  |
| urlId | string | 必須 |  |
| fromCommentId | string | 任意 |  |
| viewed | bool | 任意 |  |

## レスポンス

返却: [`Option[GetNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count200response.nim)

## 例

[inline-code-attrs-start title = 'getNotificationCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(
  tenantId = "acme-corp-tenant-12",
  userId = "user-84",
  urlId = "news/2026/market-update",
  fromCommentId = "cmt-20251234",
  viewed = false
)

if response.isSome:
  let notificationData = response.get()
  echo "Received notification data"
else:
  echo "No notification data"
[inline-code-end]

---