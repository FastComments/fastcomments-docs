## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetTenantsOptions | Ne |  |

## Odgovor

Vraća: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Primer

[inline-code-attrs-start title = 'getTenants Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenants(tenantId = "my-tenant-123", options = GetTenantsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]