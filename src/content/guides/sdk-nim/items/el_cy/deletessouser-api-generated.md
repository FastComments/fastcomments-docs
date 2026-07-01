## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteSSOUserOptions | No |  |

## Απόκριση

Επιστρέφει: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.deleteSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteSSOUserOptions()
)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
[inline-code-end]