## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 아니요 |  |
| commentData | CommentData | 아니요 |  |
| sessionId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[SaveCommentsResponseWithPresence]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comments_response_with_presence.nim)

## 예제

[inline-code-attrs-start title = 'createCommentPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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