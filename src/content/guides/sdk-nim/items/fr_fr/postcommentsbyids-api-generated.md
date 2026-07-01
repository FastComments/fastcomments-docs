## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentsByIdsParams | CommentsByIdsParams | Non |  |
| sso | string = "" | Non |  |

## Réponse

Retourne : [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Exemple

[inline-code-attrs-start title = 'postCommentsByIds Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # utilisez la réponse selon les besoins
[inline-code-end]