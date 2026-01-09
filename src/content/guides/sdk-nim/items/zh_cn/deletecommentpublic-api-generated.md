## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[DeleteCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'deleteCommentPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  echo "Delete succeeded"
  echo "HTTP status: ", httpResponse.status
else:
  echo "Delete failed, HTTP status: ", httpResponse.status
[inline-code-end]

---