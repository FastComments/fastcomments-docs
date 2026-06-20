## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetCachedNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_cached_notification_count_response.nim)

## 例

[inline-code-attrs-start title = 'getCachedNotificationCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCachedNotificationCount(tenantId = "my-tenant-123", id = "notification-789")
if response.isSome:
  let cached = response.get()
  echo "Cached notification count: ", $cached
else:
  echo "No cached notification count"
[inline-code-end]

---