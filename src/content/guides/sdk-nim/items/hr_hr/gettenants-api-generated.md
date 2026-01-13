## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| meta | string | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetTenants_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getTenants'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "include=details", skip = 0.0)
if response.isSome:
  let tenants = response.get()
  echo "Received tenants: ", repr(tenants)
else:
  echo "Failed to retrieve tenants"
[inline-code-end]

---