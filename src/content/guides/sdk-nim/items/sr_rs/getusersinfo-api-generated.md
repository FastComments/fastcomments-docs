Обавештаји о корисницима у групи за једног станара (tenant). За дате userIds, враћа приказ информација из User / SSOUser.
Користи га виџет за коментаре да обогати кориснике који су управо појавили преко догађаја присуства.
Без контекста странице: приватност се спроводи једнолико (приватни профили су маскирани).

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Не |  |

## Одговор

Враћа: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]