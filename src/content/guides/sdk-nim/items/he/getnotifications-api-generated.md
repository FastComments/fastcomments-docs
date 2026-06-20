## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | לא |  |
| urlId | string | כן |  |
| fromCommentId | string | לא |  |
| viewed | bool | לא |  |
| skip | float64 | לא |  |

## תגובה

מחזיר: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(tenantId = "my-tenant-123", userId = "user-456", urlId = "news/article-title", fromCommentId = "cmt-789", viewed = false, skip = 0.0)
if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---