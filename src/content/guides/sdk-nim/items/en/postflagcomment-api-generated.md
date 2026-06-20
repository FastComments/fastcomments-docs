## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'postFlagComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postFlagComment(commentId = "comment-742", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Comment flagged successfully"
else:
  echo "Failed to flag comment"
[inline-code-end]
