## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text200response.nim)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-987654321", editKey = "", sso = "")
if response.isSome:
  let comment = response.get()
[inline-code-end]
