## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| pageSize | int | 아니요 |  |
| afterId | string | 아니요 |  |
| includeContext | bool | 아니요 |  |
| afterCreatedAt | int64 | 아니요 |  |
| unreadOnly | bool | 아니요 |  |
| dmOnly | bool | 아니요 |  |
| noDm | bool | 아니요 |  |
| includeTranslations | bool | 아니요 |  |
| includeTenantNotifications | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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