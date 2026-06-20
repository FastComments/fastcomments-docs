## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| voteId | string | No |  |
| urlId | string | Sí |  |
| broadcastId | string | No |  |
| editKey | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  voteId = "vote-789",
  urlId = "news/article-title",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let voteResp = response.get()
  echo "Vote delete response:", voteResp
else:
  echo "No response body, HTTP response:", httpResponse
[inline-code-end]

---