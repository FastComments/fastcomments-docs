## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | float64 | Nee |  |
| monthNumber | float64 | Nee |  |
| dayNumber | float64 | Nee |  |
| skip | float64 | Nee |  |

## Respons

Geeft terug: [`Option[GetTenantDailyUsages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantDailyUsages Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  yearNumber = 2025.0,
  monthNumber = 6.0,
  dayNumber = 15.0,
  skip = 0.0
)

if response.isSome:
  let usages = response.get()
  echo usages
else:
  echo "No daily usages returned"
[inline-code-end]