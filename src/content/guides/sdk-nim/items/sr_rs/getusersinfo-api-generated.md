---
Скуп информација о корисницима за tenant. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи га видгет за коментаре да обогати кориснике који су се управо појавили путем догађаја присуства.
Нема контекста странице: приватност се доследно примењује (приватни профили су маскирани).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Не |  |

## Одговор

Враћа: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---