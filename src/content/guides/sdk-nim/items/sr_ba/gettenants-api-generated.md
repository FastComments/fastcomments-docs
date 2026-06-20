## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| meta | string | Не |  |
| skip | float64 | Не |  |

## Одговор

Враћа: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Примјер

[inline-code-attrs-start title = 'getTenants Примјер'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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