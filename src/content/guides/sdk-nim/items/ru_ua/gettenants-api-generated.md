## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenantId | string | Так |  |
| options | GetTenantsOptions | Ні |  |

## Відповідь

Повертає: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Приклад

[inline-code-attrs-start title = 'getTenants Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenants(tenantId = "my-tenant-123", options = GetTenantsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]