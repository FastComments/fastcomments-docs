## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| meta | string | Не |  |
| skip | float64 | Не |  |

## Отговор

Връща: [`Option[GetTenants_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getTenants'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "include=details", skip = 0.0)
if response.isSome:
  let tenants = response.get()
  echo "Received tenants: ", repr(tenants)
else:
  echo "Failed to retrieve tenants"
[inline-code-end]

---