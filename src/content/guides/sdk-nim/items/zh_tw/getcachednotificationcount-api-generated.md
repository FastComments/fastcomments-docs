## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

回傳: [`Option[GetCachedNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count200response.nim)

## 範例

[inline-code-attrs-start title = 'getCachedNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notif-thread-2026")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
[inline-code-end]

---