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

Devuelve: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-789",
  voteId = "",
  urlId = "news/breaking-story-2025",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Vote removed for comment cmt-789"
else:
  echo "No response body returned"
[inline-code-end]

---