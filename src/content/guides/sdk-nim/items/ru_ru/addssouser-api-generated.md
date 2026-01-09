## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPISSOUserData | CreateAPISSOUserData | Нет |  |

## Ответ

Возвращает: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример addSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addSSOUser(
  tenantId = "my-tenant-123",
  createAPISSOUserData = CreateAPISSOUserData(
    id = "sso-456",
    email = "alice.johnson@newsorg.com",
    name = "Alice Johnson",
    roles = @["editor", "contributor"],
    isActive = true,
    isAdmin = false
  )
)
if response.isSome:
  let apiResp = response.get()
  discard apiResp
else:
  discard httpResponse
[inline-code-end]

---