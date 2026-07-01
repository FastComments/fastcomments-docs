## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Antwort

Rückgabe: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getCommentBanStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banStatusOpt, httpResp) = client.getCommentBanStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  sso = "")

if banStatusOpt.isSome:
  let banStatus = banStatusOpt.get()
  echo banStatus
[inline-code-end]