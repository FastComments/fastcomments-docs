## Parameter

| Name               | Typ                     | Erforderlich | Beschreibung |
|--------------------|------------------------|--------------|--------------|
| tenantId           | string                 | Ja           |  |
| commentsByIdsParams| CommentsByIdsParams    | Nein         |  |
| sso                | string = ""            | Nein         |  |

## Antwort

Rückgabe: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postCommentsByIds Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # Verwende resp nach Bedarf
[inline-code-end]