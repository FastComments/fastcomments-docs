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
  commentId = "cmt-987654",
  urlId = "news/article-2025-breaking",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  sessionId = "",
  sso = ""
)

if response.isSome:
  let voteResult = response.get()
  discard voteResult
else:
  discard httpResponse
[inline-code-end]
