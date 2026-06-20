## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 否 |  |
| commentData | CommentData | 否 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[SaveCommentsResponseWithPresence]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comments_response_with_presence.nim)

## 示例

[inline-code-attrs-start title = 'createCommentPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentPayload = CommentData(
  text = "Great write-up on serverless architectures.",
  authorName = "Jane Doe",
  authorEmail = "jane.doe@example.com",
  isPublic = true,
  tags = @["tech", "serverless"]
)
let (response, httpResponse) = client.createCommentPublic(
  tenantId = "my-tenant-123",
  urlId = "news/2026/06/fastcomments-sdk-update",
  broadcastId = "broadcast-2026-06-19",
  commentData = commentPayload,
  sessionId = "sess-8a7b6c",
  sso = "sso-jwt-abc123"
)
if response.isSome:
  let saved = response.get()
  discard saved
[inline-code-end]

---