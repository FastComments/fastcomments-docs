## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'getBanUsersFromComment Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(tenantId = "my-tenant-001", commentId = "cmt-123456", sso = "")
if response.isSome:
  let banInfo = response.get()
  echo banInfo
[inline-code-end]

---