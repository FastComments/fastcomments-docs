## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| pageSize | int | Нет |  |
| afterId | string | Нет |  |
| includeContext | bool | Нет |  |
| afterCreatedAt | int64 | Нет |  |
| unreadOnly | bool | Нет |  |
| dmOnly | bool | Нет |  |
| noDm | bool | Нет |  |
| includeTranslations | bool | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  pageSize = 50,
  afterId = "notif_9a1b2c3d",
  includeContext = true,
  afterCreatedAt = int64(1699999999000),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  sso = ""
)
if response.isSome:
  let notifications = response.get()
  discard notifications
else:
  discard httpResponse
[inline-code-end]

---