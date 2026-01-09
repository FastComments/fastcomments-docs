## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| broadcastId | string | No |  |
| commentData | CommentData | No |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[CreateCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_comment_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createCommentPublic(
  tenantId = "my-tenant-123",
  urlId = "news/breaking-elections-2025",
  broadcastId = "broadcast-456",
  commentData = CommentData(
    content = "Great reporting — thanks for the clear analysis!",
    authorName = "Jane Doe",
    authorEmail = "jane.doe@example.com",
    isVerified = false,
    tags = @["politics", "analysis"]
  ),
  sessionId = "session-789",
  sso = "sso-token-abc123"
)

if response.isSome:
  let created = response.get()
  echo "Created comment:", created
else:
  echo "No comment returned, HTTP status: ", httpResponse.status`
[inline-code-end]

---