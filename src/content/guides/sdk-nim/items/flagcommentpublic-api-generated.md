## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'flagCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-8f3a21", isFlagged = false, sso = "")
if response.isSome:
  let flagResult = response.get()
  echo "FlagCommentPublic succeeded:", flagResult
else:
  echo "FlagCommentPublic returned no data, HTTP status:", httpResponse.status.code
[inline-code-end]
