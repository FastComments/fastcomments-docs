## Parámetros

| Name | Type | Obligatorio | Descripción |
|------|------|------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| urlId | string | Sí |  |
| broadcastId | string | No |  |
| voteBodyParams | VoteBodyParams | No |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de voteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  urlId = "news/article-2026-inflation",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  sessionId = "",
  sso = ""
)

if response.isSome:
  let voteResp = response.get()
  discard voteResp
else:
  discard httpResponse
[inline-code-end]

---