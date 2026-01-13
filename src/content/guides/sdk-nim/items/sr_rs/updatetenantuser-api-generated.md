## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateTenantUserBody | UpdateTenantUserBody | Не |  |
| updateComments | string | Не |  |

## Одговор

Враћа: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
var body: UpdateTenantUserBody
body.email = "jane.doe@example.com"
body.displayName = "Jane Doe"
body.roles = @["moderator", "editor"]
body.isActive = true

let (response, httpResponse) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateTenantUserBody = body,
  updateComments = "Promoted user to moderator and editor roles"
)

if response.isSome:
  let flagResp = response.get()
  echo flagResp
else:
  echo "Update failed, status: ", httpResponse.status
[inline-code-end]