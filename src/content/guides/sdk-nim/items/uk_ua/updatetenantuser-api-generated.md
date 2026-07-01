## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateTenantUserBody | UpdateTenantUserBody | Ні |  |
| updateComments | string = "" | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'updateTenantUser Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantUserBody()
let (optResp, httpResp) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateTenantUserBody = updateBody,
  updateComments = "Changed role to moderator",
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]