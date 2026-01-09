## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Respuesta

Devuelve: [`Option[DeleteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-456abc", contextUserId = "user-789", isLive = true)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Delete succeeded"
else:
  echo "No delete response"
[inline-code-end]

---