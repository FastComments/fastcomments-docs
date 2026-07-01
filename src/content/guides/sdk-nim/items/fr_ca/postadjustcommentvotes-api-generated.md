## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Non |  |
| options | PostAdjustCommentVotesOptions | Non |  |

## Réponse

Retourne : [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (adjustRespOpt, httpResp) = client.postAdjustCommentVotes(
  tenantId = "my-tenant-123",
  commentId = "cmt-789",
  adjustCommentVotesParams = AdjustCommentVotesParams(),
  options = PostAdjustCommentVotesOptions()
)

if adjustRespOpt.isSome:
  let adjustResp = adjustRespOpt.get()
[inline-code-end]