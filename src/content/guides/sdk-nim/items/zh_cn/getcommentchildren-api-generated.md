## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## 示例

[inline-code-attrs-start title = 'getCommentChildren 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentChildren(commentId = "comment-98765", sso = "")
if response.isSome:
  let childResp = response.get()
  discard childResp
[inline-code-end]

---