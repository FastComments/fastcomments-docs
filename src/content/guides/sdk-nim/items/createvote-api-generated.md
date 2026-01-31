## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Example

[inline-code-attrs-start title = 'createVote Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  direction = "",
  userId = "",
  anonUserId = ""
)
if response.isSome:
  let vote = response.get()
  echo "Vote response:", $vote
[inline-code-end]
