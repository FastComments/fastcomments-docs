## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| banUserUndoParams | BanUserUndoParams | Nee |  |
| sso | string = "" | Nee |  |

## Reactie

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserUndo Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postBanUserUndo(
  tenantId = "my-tenant-123",
  banUserUndoParams = BanUserUndoParams(userId = "user-456"),
  sso = ""
)

if apiResp.isSome:
  let _ = apiResp.get()
[inline-code-end]