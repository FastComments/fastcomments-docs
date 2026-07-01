---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Não |  |
| updateComments | bool | Não |  |

## Resposta

Retorna: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'patchSSOUser Exemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPISSOUserData()
let (responseOpt, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = updateData,
  updateComments = true)
if responseOpt.isSome:
  let response = responseOpt.get()
  echo response
echo httpResponse.statusCode
[inline-code-end]

---