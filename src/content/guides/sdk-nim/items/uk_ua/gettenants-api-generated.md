## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| meta | string | Ні |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenants'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "env=production", skip = 0.0)
if response.isSome:
  let tenantsResp = response.get()
  discard tenantsResp
  echo "Tenants fetched successfully"
else:
  echo "Request failed with status ", httpResponse.status
[inline-code-end]

---