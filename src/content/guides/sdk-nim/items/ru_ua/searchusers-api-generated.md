---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| usernameStartsWith | string | Нет |  |
| mentionGroupIds | seq[string] | Нет |  |
| sso | string | Нет |  |
| searchSection | string | Нет |  |

## Ответ

Возвращает: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/2026/ai-product-launch",
  usernameStartsWith = "",
  mentionGroupIds = @[],
  sso = "",
  searchSection = ""
)
if response.isSome:
  let users = response.get()
  echo "Received users:", users.toString()
[inline-code-end]

---