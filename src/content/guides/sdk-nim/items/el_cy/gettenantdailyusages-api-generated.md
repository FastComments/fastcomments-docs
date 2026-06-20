## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| yearNumber | float64 | Όχι |  |
| monthNumber | float64 | Όχι |  |
| dayNumber | float64 | Όχι |  |
| skip | float64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'getTenantDailyUsages Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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