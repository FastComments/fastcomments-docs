## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| pageSize | int | לא |  |
| afterId | string | לא |  |
| includeContext | bool | לא |  |
| afterCreatedAt | int64 | לא |  |
| unreadOnly | bool | לא |  |
| dmOnly | bool | לא |  |
| noDm | bool | לא |  |
| includeTranslations | bool | לא |  |
| includeTenantNotifications | bool | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  pageSize = 0,
  afterId = "",
  includeContext = false,
  afterCreatedAt = 0,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  includeTenantNotifications = false,
  sso = ""
)

if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---