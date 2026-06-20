## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | float64 | Nein |  |
| monthNumber | float64 | Nein |  |
| dayNumber | float64 | Nein |  |
| skip | float64 | Nein |  |

## Antwort

Gibt zurück: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTenantDailyUsages Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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