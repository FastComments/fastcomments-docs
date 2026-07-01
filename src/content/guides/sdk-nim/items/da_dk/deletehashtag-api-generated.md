## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| tag | string | Nej |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nej |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteHashTag(tenantId = "my-tenant-123", tag = "sports", deleteHashTagRequestBody = DeleteHashTagRequestBody())
if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]