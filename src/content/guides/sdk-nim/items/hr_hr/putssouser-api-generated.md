## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Ne |  |
| updateComments | bool | Ne |  |

## Odgovor

Vraća: [`Option[PutSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_sso_user_api_response.nim)

## Primjer

[inline-code-attrs-start title = 'putSSOUser Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateAPISSOUserData = default(UpdateAPISSOUserData),
  updateComments = false)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
  echo apiResp
[inline-code-end]