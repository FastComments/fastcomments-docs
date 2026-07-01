## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|---------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | UnFlagCommentOptions | No |  |

## Antwort

Rückgabe: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Beispiel

[inline-code-attrs-start title = 'unFlagComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (flagRespOpt, httpResp) = client.unFlagComment(tenantId = "my-tenant-123", id = "comment-456", options = UnFlagCommentOptions())
if flagRespOpt.isSome:
  let flagResp = flagRespOpt.get()
[inline-code-end]