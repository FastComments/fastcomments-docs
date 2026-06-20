## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| meta | string | Nee |  |
| skip | float64 | Nee |  |

## Antwoord

Retourneert: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTenants Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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