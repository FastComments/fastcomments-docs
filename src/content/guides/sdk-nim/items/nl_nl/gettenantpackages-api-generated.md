## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| skip | float64 | Nee |  |

## Antwoord

Retourneert: [`Option[GetTenantPackagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantPackages Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let packages = response.get()
  echo "Received tenant packages:"
  echo packages
else:
  echo "No packages found for tenant 'my-tenant-123'"
[inline-code-end]

---