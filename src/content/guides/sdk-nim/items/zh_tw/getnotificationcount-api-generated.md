## 參數

| 名稱 | Type | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 是 |  |
| fromCommentId | string | 否 |  |
| viewed | bool | 否 |  |

## 回應

回傳: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(tenantId = "my-tenant-123", userId = "user-987", urlId = "news/2026/06/election-results", fromCommentId = "", viewed = false)
if response.isSome:
  let notifyData = response.get()
  echo notifyData
[inline-code-end]

---