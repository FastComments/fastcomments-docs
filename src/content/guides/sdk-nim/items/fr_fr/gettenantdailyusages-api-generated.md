## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| yearNumber | float64 | Non |  |
| monthNumber | float64 | Non |  |
| dayNumber | float64 | Non |  |
| skip | float64 | Non |  |

## Réponse

Renvoie : [`Option[GetTenantDailyUsages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple pour getTenantDailyUsages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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