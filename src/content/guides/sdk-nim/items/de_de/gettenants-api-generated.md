## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| options | GetTenantsOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetTenantsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTenants Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenants(tenantId = "my-tenant-123", options = GetTenantsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]

---