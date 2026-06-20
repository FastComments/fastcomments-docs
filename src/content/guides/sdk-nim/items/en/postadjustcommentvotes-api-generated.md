## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Example

[inline-code-attrs-start title = 'postAdjustCommentVotes Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]
