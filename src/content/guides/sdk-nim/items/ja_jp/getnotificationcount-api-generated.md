## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetNotificationCountOptions | No |  |

## レスポンス

返り値: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## 例

[inline-code-attrs-start title = 'getNotificationCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotificationCount(tenantId = "my-tenant-123", options = GetNotificationCountOptions())
if notifOpt.isSome:
  let notif = notifOpt.get()
  echo notif
[inline-code-end]