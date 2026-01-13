## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| deleteComments | string | Não |  |
| commentDeleteMode | string | Não |  |

## Resposta

Retorna: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-456", deleteComments = "", commentDeleteMode = "")
if response.isSome:
  let flagResp = response.get()
  echo flagResp
[inline-code-end]