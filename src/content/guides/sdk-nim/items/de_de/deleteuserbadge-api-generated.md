## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Rückgabe: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteUserBadge Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if apiResponse.isSome:
  let success = apiResponse.get()
[inline-code-end]