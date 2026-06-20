Lista las páginas de un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas.
Requiere `enableFChat` sea true en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso por grupo del usuario que realiza la solicitud.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional de prefijo de título, sin distinguir mayúsculas/minúsculas. |
| sortBy | string | query | No | Orden. `updatedAt` (por defecto, primero los más recientes), `commentCount` (primero los con más comentarios), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devolver páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
  limit: 56, # Integer | 1..200, default 50
  q: 'q_example', # String | Optional case-insensitive title prefix filter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
  has_comments: true # Boolean | If true, only return pages with at least one comment.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]