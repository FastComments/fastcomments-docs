## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Sì |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---