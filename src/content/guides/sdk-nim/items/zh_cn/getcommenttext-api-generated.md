## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[GetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text200response.nim)

## 示例

[inline-code-attrs-start title = 'getCommentText 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-456789", editKey = "", sso = "")

if response.isSome:
  let comment = response.get()
  echo "Comment text: ", $comment
else:
  echo "No comment returned"
[inline-code-end]

---