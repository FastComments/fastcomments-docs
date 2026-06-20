---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Non |  |
| sso | string | Non |  |

## Réponse

Retourne: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---