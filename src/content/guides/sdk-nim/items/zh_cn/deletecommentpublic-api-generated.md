## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## 示例

[inline-code-attrs-start title = 'deleteCommentPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", editKey = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete acknowledged, HTTP status: ", httpResponse.status
[inline-code-end]