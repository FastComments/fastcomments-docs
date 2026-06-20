## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| editKey | string | No |  |
| sso | string | No |  |

## Resposta

Retorna: [`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", editKey = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete acknowledged, HTTP status: ", httpResponse.status
[inline-code-end]

---