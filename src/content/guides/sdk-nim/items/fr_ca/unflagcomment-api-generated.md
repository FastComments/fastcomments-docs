## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | UnFlagCommentOptions | No |  |

## Réponse

Renvoie : [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple unFlagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (flagRespOpt, httpResp) = client.unFlagComment(tenantId = "my-tenant-123", id = "comment-456", options = UnFlagCommentOptions())
if flagRespOpt.isSome:
  let flagResp = flagRespOpt.get()
[inline-code-end]