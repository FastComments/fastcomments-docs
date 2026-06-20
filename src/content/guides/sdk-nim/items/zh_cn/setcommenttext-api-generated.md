## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 否 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## 示例

[inline-code-attrs-start title = 'setCommentText 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  commentTextUpdateRequest = CommentTextUpdateRequest(text: "Updated comment text to fix a typo and clarify meaning."),
  editKey = "",
  sso = ""
)
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---