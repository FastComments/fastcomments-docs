## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| commentData | CommentData | No |  |
| options | CreateCommentPublicOptions | No |  |

## Απόκριση

Επιστρέφει: [`Option[SaveCommentsResponseWithPresence]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_save_comments_response_with_presence.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'createCommentPublic Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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