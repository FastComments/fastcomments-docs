## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| pageSize | int | Нет |  |
| afterId | string | Нет |  |
| includeContext | bool | Нет |  |
| afterCreatedAt | int64 | Нет |  |
| unreadOnly | bool | Нет |  |
| dmOnly | bool | Нет |  |
| noDm | bool | Нет |  |
| includeTranslations | bool | Нет |  |
| includeTenantNotifications | bool | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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