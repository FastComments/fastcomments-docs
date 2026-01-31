## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | int | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetCommentVoteUserNames_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names200response.nim)

## Example

[inline-code-attrs-start title = 'getCommentVoteUserNames Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(
  tenantId = "my-tenant-123",
  commentId = "cmt-7890",
  dir = 0,
  sso = ""
)

if response.isSome:
  let result = response.get()
  echo "Received vote user names response: ", $result
[inline-code-end]
