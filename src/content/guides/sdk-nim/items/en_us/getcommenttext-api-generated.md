## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-987654321", editKey = "", sso = "")

if response.isSome:
  let commentTextResp = response.get()
  echo commentTextResp
else:
  echo "No comment text returned"
[inline-code-end]
