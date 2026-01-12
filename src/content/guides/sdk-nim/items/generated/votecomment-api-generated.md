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
let voteBody = VoteBodyParams(vote = 1)
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  urlId = "news/article-2026-election",
  broadcastId = "",
  voteBodyParams = voteBody,
  sessionId = "",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Vote response:", $result
[inline-code-end]
