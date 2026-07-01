## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Nein |  |
| options | PostAdjustCommentVotesOptions | Nein |  |

## Antwort

Rückgabe: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postAdjustCommentVotes Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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