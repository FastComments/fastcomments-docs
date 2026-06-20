## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de blockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-7890",
  blockFromCommentParams = BlockFromCommentParams(
    reason = "Repeated abusive language",
    durationMinutes = 1440,
    notifyUser = true,
    tags = @["abuse", "automated"]
  ),
  userId = "user-456",
  anonUserId = ""
)

if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse
[inline-code-end]