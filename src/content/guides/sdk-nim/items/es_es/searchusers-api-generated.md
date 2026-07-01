## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| options | SearchUsersOptions | No |  |

## Respuesta

Devuelve: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## Ejemplo

[inline-code-attrs-start title = 'searchUsers Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = SearchUsersOptions()
)

if response.isSome:
  let result = response.get()
[inline-code-end]

---