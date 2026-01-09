## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Nein |  |
| commentData | CommentData | Nein |  |
| sessionId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[CreateCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_comment_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'createCommentPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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