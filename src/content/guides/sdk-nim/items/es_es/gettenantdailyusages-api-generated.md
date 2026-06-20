## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| yearNumber | float64 | No |  |
| monthNumber | float64 | No |  |
| dayNumber | float64 | No |  |
| skip | float64 | No |  |

## Respuesta

Devuelve: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTenantDailyUsages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  yearNumber = 2026.0,
  monthNumber = 6.0,
  dayNumber = 19.0,
  skip = 0.0
)

if response.isSome:
  let usage = response.get()
  discard usage
[inline-code-end]

---