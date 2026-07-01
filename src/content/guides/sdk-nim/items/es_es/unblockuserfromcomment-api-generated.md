## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | No |  |
| options | UnBlockUserFromCommentOptions | No |  |

## Respuesta

Devuelve: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  unBlockFromCommentParams = UnBlockFromCommentParams(userId = "user-789", commentId = "cmt-321"),
  options = UnBlockUserFromCommentOptions(),
)

if response.isSome:
  let unblockSuccess = response.get()
[inline-code-end]