## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Όχι |  |
| voteBodyParams | VoteBodyParams | Όχι |  |
| sessionId | string | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα voteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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