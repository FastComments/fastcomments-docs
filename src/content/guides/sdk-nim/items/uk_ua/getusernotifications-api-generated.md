## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| pageSize | int | Ні |  |
| afterId | string | Ні |  |
| includeContext | bool | Ні |  |
| afterCreatedAt | int64 | Ні |  |
| unreadOnly | bool | Ні |  |
| dmOnly | bool | Ні |  |
| noDm | bool | Ні |  |
| includeTranslations | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[GetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notifications200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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