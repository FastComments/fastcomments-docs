## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentsByIdsParams | CommentsByIdsParams | Ne |  |
| sso | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Primjer

[inline-code-attrs-start title = 'postCommentsByIds primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # koristite odgovor po potrebi
[inline-code-end]