## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| urlId | string | Tak |  |
| broadcastId | string | Nie |  |
| voteBodyParams | VoteBodyParams | Nie |  |
| sessionId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład voteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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