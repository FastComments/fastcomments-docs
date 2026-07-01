## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Não |  |
| sso | string = "" | Não |  |

## Resposta

Retorna: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo blockFromCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (blockOpt, httpResp) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if blockOpt.isSome:
  let block = blockOpt.get()
[inline-code-end]