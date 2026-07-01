## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | No |  |
| options | PostAdjustCommentVotesOptions | No |  |

## Response

Returns: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Example

[inline-code-attrs-start title = 'postAdjustCommentVotes Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
