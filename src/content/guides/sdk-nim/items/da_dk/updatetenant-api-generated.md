## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |
| updateTenantBody | UpdateTenantBody | Nej |  |

## Response

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenant(
  tenantId = "my-tenant-123",
  id = "settings",
  updateTenantBody = UpdateTenantBody(
    name = "My Tenant 123",
    enableModeration = true,
    allowedDomains = @["news.example.com", "blog.example.org"],
    maxCommentLength = 1000
  )
)

if response.isSome:
  let apiResp = response.get()
  echo "Tenant updated successfully: ", apiResp
else:
  echo "Failed to update tenant, HTTP status: ", httpResponse.status
[inline-code-end]