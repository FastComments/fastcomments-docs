## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## Primjer

[inline-code-attrs-start title = 'getCommentBanStatus Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banStatusOpt, httpResp) = client.getCommentBanStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  sso = "")

if banStatusOpt.isSome:
  let banStatus = banStatusOpt.get()
  echo banStatus
[inline-code-end]