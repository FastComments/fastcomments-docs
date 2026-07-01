## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| options | GetCommentTextOptions | Não |  |

## Resposta

Retorna: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = GetCommentTextOptions()
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
[inline-code-end]