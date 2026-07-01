## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantBody | UpdateTenantBody | No |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'updateTenant Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantBody(
  name: "My Tenant",
  description: "Tenant for news site",
  isActive: true,
  allowedDomains: @["example.com", "news.com"]
)

let (apiResp, httpResp) = client.updateTenant(
  tenantId = "my-tenant-123",
  id = "tenant-456",
  updateTenantBody = updateBody
)

if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]

---