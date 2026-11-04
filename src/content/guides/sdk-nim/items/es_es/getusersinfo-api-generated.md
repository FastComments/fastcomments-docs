Información masiva de usuarios para un tenant. Dados userIds, devuelve la información de visualización de User / SSOUser.
Usado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de manera uniforme (los perfiles privados se enmascaran).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| ids | string | No |  |

## Respuesta

Devuelve: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]