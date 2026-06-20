## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateTenantUserBody | UpdateTenantUserBody | Ні |  |
| updateComments | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання updateTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-987",
  updateTenantUserBody = UpdateTenantUserBody(
    displayName = "Jane Doe",
    email = "jane.doe@example.com",
    roles = @["moderator", "editor"],
    isActive = true
  ),
  updateComments = "true"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---