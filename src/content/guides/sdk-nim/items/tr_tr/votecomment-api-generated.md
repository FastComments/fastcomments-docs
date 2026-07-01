## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| voteBodyParams | VoteBodyParams | No |  |
| options | VoteCommentOptions | No |  |

## Response

Döndürür: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Example

[inline-code-attrs-start title = 'voteComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteRespOpt, httpResp) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  urlId = "blog/how-to-code",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  options = VoteCommentOptions()
)

if voteRespOpt.isSome:
  let voteResp = voteRespOpt.get()
[inline-code-end]