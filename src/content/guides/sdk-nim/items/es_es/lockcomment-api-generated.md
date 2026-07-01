## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo lockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (lockResult, httpRes) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  broadcastId = "",
  sso = "")

if lockResult.isSome:
  let resp = lockResult.get()
  discard resp
[inline-code-end]