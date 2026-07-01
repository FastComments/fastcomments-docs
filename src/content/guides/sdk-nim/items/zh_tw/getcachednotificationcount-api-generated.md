## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 回應

返回：[`Option[GetCachedNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getCachedNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (cachedCountOpt, httpResp) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "article-456")
if cachedCountOpt.isSome:
  let cachedCount = cachedCountOpt.get()
  echo cachedCount
[inline-code-end]