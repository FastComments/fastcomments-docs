## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| redirectURL | string = "" | Nee |  |

## Respons

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'sendLoginLink Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.sendLoginLink(
  tenantId = "my-tenant-123",
  id = "user-456",
  redirectURL = "https://myapp.example.com/login-success"
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]