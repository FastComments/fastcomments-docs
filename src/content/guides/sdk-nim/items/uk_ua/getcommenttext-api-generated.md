## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | GetCommentTextOptions | No |  |

## Response

Returns: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## Example

[inline-code-attrs-start title = 'Приклад getCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = GetCommentTextOptions()
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
[inline-code-end]