## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Ne |  |
| sso | string | Ne |  |

## Odziv

Vrača: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---