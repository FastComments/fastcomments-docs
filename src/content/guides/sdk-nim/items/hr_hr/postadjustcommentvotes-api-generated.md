## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]