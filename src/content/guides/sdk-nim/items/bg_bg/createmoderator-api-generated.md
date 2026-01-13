## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createModeratorBody | CreateModeratorBody | Не |  |

## Отговор

Връща: [`Option[CreateModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за createModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateModeratorBody(
  email = "moderator@news-site.com",
  displayName = "News Moderator",
  permissions = @["approve_comments", "delete_comments"],
  isSuperAdmin = false
)

let (response, httpResponse) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = createBody)

if response.isSome:
  let moderator = response.get()
  echo "Created moderator: ", $moderator
[inline-code-end]

---