## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostFlagCommentOptions | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'postFlagComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = PostFlagCommentOptions()
let (response, httpResponse) = client.postFlagComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = opts,
)
if response.isSome:
  let result = response.get()
[inline-code-end]
