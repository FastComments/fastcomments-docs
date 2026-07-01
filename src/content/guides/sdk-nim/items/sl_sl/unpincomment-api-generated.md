## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string = "" | Ne |  |

## Odgovor

Vrne: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer unPinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.unPinComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "broadcast-001",
  sso = ""
)

if responseOpt.isSome:
  let resp = responseOpt.get()
  echo resp
[inline-code-end]