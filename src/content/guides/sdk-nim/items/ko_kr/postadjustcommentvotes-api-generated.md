## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 아니오 |  |
| options | PostAdjustCommentVotesOptions | 아니오 |  |

## Response

반환: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Example

[inline-code-attrs-start title = 'postAdjustCommentVotes 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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