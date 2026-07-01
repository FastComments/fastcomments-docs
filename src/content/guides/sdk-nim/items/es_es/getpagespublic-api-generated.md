Lista de páginas para un inquilino. Utilizado por el cliente de escritorio FChat para poblar su lista de salas.  
Requiere `enableFChat` en **true** en la configuración personalizada resuelta para cada página.  
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario solicitante.

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| options | GetPagesPublicOptions | No |  |

## Respuesta

Devuelve: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]