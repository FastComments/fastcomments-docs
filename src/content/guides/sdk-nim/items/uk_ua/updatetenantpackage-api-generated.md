## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'updateTenantPackage Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---