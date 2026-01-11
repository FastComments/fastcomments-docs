## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| commentData | CommentData | No |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[CreateCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createCommentPublic(
  tenantId = "my-tenant-123",
  urlId = "news/2025/ai-ethics",
  broadcastId = "broadcast-456",
  commentData = CommentData(
    content = "Insightful article about AI governance and policy implications.",
    authorName = "Jane Doe",
    authorEmail = "jane.doe@example.com",
    isPublic = true,
    tags = @["ai", "ethics"]
  ),
  sessionId = "sess-789",
  sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]
