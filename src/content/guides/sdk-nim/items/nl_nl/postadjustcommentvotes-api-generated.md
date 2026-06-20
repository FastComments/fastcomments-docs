## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Nee |  |
| sso | string | Nee |  |

## Antwoord

Retourneert: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postAdjustCommentVotes Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---