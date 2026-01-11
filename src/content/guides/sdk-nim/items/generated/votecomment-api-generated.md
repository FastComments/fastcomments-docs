## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| voteBodyParams | VoteBodyParams | No |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Example

[inline-code-attrs-start title = 'voteComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-98765",
  urlId = "news/2025/tech/ai-advance",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  sessionId = "",
  sso = ""
)
if response.isSome:
  let voteResp = response.get()
  echo "Vote succeeded: ", $voteResp
else:
  echo "Vote failed, HTTP status: ", $httpResponse.status
[inline-code-end]
