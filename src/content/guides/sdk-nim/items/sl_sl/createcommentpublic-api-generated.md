## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| commentData | CommentData | No |  |
| options | CreateCommentPublicOptions | No |  |

## Odgovor

Vrne: [`Option[SaveCommentsResponseWithPresence]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comments_response_with_presence.nim)

## Primer

[inline-code-attrs-start title = 'createCommentPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let
  tenantId = "my-tenant-123"
  urlId = "news/article-title"
  broadcastId = "broadcast-456"
  commentData = CommentData()
  options = CreateCommentPublicOptions()
let (maybeResp, httpResp) = client.createCommentPublic(
  tenantId = tenantId,
  urlId = urlId,
  broadcastId = broadcastId,
  commentData = commentData,
  options = options)

if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]