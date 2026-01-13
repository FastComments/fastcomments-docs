## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Не |  |
| updateComments | bool | Не |  |

## Одговор

Враћа: [`Option[PutSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_sso_user_api_response.nim)

## Пример

[inline-code-attrs-start title = 'putSSOUser Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateAPISSOUserData = UpdateAPISSOUserData(
    externalId = "ext-789",
    displayName = "Jane Doe",
    email = "jane.doe@example.com",
    avatarUrl = "https://cdn.news-site.com/avatars/jane.jpg",
    roles = @["member", "subscriber"]
  ),
  updateComments = true
)

if response.isSome:
  let result = response.get()
  echo "SSO user updated:", result
[inline-code-end]

---