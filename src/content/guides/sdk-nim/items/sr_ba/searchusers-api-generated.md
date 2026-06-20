## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| usernameStartsWith | string | Не |  |
| mentionGroupIds | seq[string] | Не |  |
| sso | string | Не |  |
| searchSection | string | Не |  |

## Одговор

Враћа: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/top-story",
  usernameStartsWith = "",
  mentionGroupIds = @[],
  sso = "",
  searchSection = ""
)

if response.isSome:
  let searchResult = response.get()
  echo "SearchUsersResult:", searchResult
else:
  echo "No result or error. HTTP response:", httpResponse
[inline-code-end]