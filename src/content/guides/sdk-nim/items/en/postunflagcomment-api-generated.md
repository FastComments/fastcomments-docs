## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'postUnFlagComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postUnFlagComment(commentId = "comment-8f3a2b4e", sso = "")
if response.isSome:
  let apiEmpty = response.get()
  echo "Comment unflagged successfully, response: ", apiEmpty
else:
  echo "Failed to unflag comment. HTTP response: ", httpResponse
[inline-code-end]
