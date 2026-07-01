## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[GetV2PageReactUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_react_users_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getV2PageReactUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getV2PageReactUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "user-456"
)

if maybeResponse.isSome:
  let resp = maybeResponse.get()
  echo resp
[inline-code-end]