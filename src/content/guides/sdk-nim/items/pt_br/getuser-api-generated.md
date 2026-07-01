## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userOpt, httpResp) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if userOpt.isSome:
  let user = userOpt.get()
  echo user
else:
  echo "User not found"
[inline-code-end]