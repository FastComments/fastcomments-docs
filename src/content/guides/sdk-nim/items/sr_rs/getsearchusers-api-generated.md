## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| value | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Пример

[inline-code-attrs-start title = 'getSearchUsers Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---