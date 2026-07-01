## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantPackageBody | UpdateTenantPackageBody | No |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'updateTenantPackage Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateTenantPackageBody()
let (optResp, httpResp) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "premium-plan",
  updateTenantPackageBody = body
)
if optResp.isSome:
  let empty = optResp.get()
[inline-code-end]