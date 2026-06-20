## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| deleteComments | string | Nein |  |
| commentDeleteMode | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-789", deleteComments = "true", commentDeleteMode = "soft")
if response.isSome:
  let apiResp = response.get()
  echo "Tenant user deleted, response: ", apiResp
else:
  echo "Failed to delete tenant user, HTTP status: ", $httpResponse.status
[inline-code-end]