## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| tag | string | Nein |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nein |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteHashTag Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteHashTag(tenantId = "my-tenant-123", tag = "sports", deleteHashTagRequestBody = DeleteHashTagRequestBody())
if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]