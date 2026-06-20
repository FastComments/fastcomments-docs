## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 回應

Returns: [`Option[GetCachedNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getCachedNotificationCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notification-789")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
else:
  echo "No cached notification count"
[inline-code-end]

---