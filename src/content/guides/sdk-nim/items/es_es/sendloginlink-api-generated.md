## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| redirectURL | string = "" | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de sendLoginLink'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.sendLoginLink(
  tenantId = "my-tenant-123",
  id = "user-456",
  redirectURL = "https://myapp.example.com/login-success"
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]