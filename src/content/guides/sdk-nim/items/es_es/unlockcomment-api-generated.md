## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## Respuesta

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeEmpty, httpResp) = client.unLockComment(tenantId = "my-tenant-123", commentId = "comment-456", broadcastId = "", sso = "")
if maybeEmpty.isSome:
  let emptyResp = maybeEmpty.get()
  echo "Comment unlocked"
else:
  echo "Failed to unlock comment"
[inline-code-end]