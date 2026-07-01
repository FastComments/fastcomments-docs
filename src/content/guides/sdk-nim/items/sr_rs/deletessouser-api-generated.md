## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteSSOUserOptions | No |  |

## Odgovor

Vraća: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteSSOUser Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.deleteSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteSSOUserOptions()
)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
[inline-code-end]