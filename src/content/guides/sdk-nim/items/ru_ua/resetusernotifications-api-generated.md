## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Нет |  |
| afterCreatedAt | int64 | Нет |  |
| unreadOnly | bool | Нет |  |
| dmOnly | bool | Нет |  |
| noDm | bool | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[ResetUserNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  afterId = "",
  afterCreatedAt = int64(0),
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  sso = ""
)

if response.isSome:
  let result = response.get()
[inline-code-end]