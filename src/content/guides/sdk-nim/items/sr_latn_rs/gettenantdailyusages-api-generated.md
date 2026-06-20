## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| yearNumber | float64 | Ne |  |
| monthNumber | float64 | Ne |  |
| dayNumber | float64 | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTenantDailyUsages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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