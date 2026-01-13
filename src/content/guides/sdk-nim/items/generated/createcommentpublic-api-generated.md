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
  urlId = "news/my-article-2026",
  broadcastId = "",
  commentData = CommentData(),
  sessionId = "",
  sso = ""
)

if response.isSome:
  let created = response.get()
  echo "Comment created: ", $created
[inline-code-end]
