## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| pageSize | int | Не |  |
| afterId | string | Не |  |
| includeContext | bool | Не |  |
| afterCreatedAt | int64 | Не |  |
| unreadOnly | bool | Не |  |
| dmOnly | bool | Не |  |
| noDm | bool | Не |  |
| includeTranslations | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

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