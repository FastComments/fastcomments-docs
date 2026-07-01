---
Informações de usuário em lote para um tenant. Dado userIds, retorna informações de exibição de User / SSOUser.  
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.  
Sem contexto de página: a privacidade é aplicada uniformemente (perfis privados são mascarados).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Response

Retorna: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Exemplo getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]

---