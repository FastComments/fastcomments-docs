## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getCounts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCounts(sso = "sso_my-tenant-123_token_AbCdEf123456")
if response.isSome:
  let counts = response.get()
  echo counts
else:
  echo "Request failed with status:", httpResponse.status
[inline-code-end]