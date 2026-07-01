## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantPackageBody | UpdateTenantPackageBody | No |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateTenantPackageBody()
let (optResp, httpResp) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "premium-plan",
  updateTenantPackageBody = body
)
if optResp.isSome:
  let empty = optResp.get()
[inline-code-end]