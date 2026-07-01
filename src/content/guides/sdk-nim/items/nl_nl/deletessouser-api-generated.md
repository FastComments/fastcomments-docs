## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| options | DeleteSSOUserOptions | Nee |  |

## Reactie

Retourneert: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deleteSSOUser Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.deleteSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteSSOUserOptions()
)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
[inline-code-end]