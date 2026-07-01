## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Odgovor

Returns: [`Option[GetTenantPackagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getTenantPackages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if maybeResp.isSome:
  let packages = maybeResp.get()
  echo packages
  echo httpResp.statusCode
[inline-code-end]