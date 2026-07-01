## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'getCommentBanStatus Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banStatusOpt, httpResp) = client.getCommentBanStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  sso = "")

if banStatusOpt.isSome:
  let banStatus = banStatusOpt.get()
  echo banStatus
[inline-code-end]