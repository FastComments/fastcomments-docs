## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| urlId | string | Sí |  |
| broadcastId | string | No |  |
| voteBodyParams | VoteBodyParams | No |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de voteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let voteBody = VoteBodyParams()
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  urlId = "news/2025/fastcomments-integration",
  broadcastId = "",
  voteBodyParams = voteBody,
  sessionId = "",
  sso = ""
)
if response.isSome:
  let voteResp = response.get()
  echo "Vote recorded for comment cmt-456789"
else:
  echo "Failed to record vote"
[inline-code-end]

---