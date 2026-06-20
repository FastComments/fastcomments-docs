## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| pageSize | int | Не |  |
| afterId | string | Не |  |
| includeContext | bool | Не |  |
| afterCreatedAt | int64 | Не |  |
| unreadOnly | bool | Не |  |
| dmOnly | bool | Не |  |
| noDm | bool | Не |  |
| includeTranslations | bool | Не |  |
| includeTenantNotifications | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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