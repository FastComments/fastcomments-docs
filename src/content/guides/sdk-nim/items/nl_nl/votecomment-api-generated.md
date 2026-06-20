## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Nee |  |
| voteBodyParams | VoteBodyParams | Nee |  |
| sessionId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'voteComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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