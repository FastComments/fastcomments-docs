## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nej |  |
| updateModeratorBody | UpdateModeratorBody | Nej |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'updateModerator Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateModeratorBody(name = "John Doe", email = "john@example.com", isActive = true)
let (apiResult, httpResp) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = body)
if apiResult.isSome:
  let result = apiResult.get()
[inline-code-end]