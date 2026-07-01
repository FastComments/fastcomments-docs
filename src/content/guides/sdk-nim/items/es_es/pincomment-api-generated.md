## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de pinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pinResult, httpResp) = client.pinComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "broadcast-001",
  sso = "",
)

if pinResult.isSome:
  let response = pinResult.get()
  echo response
[inline-code-end]

---