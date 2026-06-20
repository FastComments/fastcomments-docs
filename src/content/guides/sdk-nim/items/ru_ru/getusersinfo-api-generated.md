---
Массовая информация о пользователях для тенанта. По списку userIds возвращает информацию для отображения из User / SSOUser.
Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие присутствия.
Нет контекста страницы: конфиденциальность соблюдается единообразно (закрытые профили скрываются).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Нет |  |

## Ответ

Возвращает: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---