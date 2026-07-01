---
Пакетная информация о пользователях для арендатора. По заданным userIds возвращается отображаемая информация из User / SSOUser. Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие присутствия. Без контекста страницы: конфиденциальность соблюдается единообразно (личные профили маскируются).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Ответ

Возвращает: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]

---