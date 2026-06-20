Lista de páginas para un tenant. Utilizado por el cliente de escritorio FChat para completar su lista de salas.
Requiere `enableFChat` sea true en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso de grupos del usuario solicitante.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, predeterminado 50 |
| q | string | query | No | Filtro opcional por prefijo de título sin distinguir mayúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, devolver solo páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor infórmalo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor de paginación opaco devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. (opcional)
let limit = 987 // Int | 1..200, predeterminado 50 (opcional)
let q = "q_example" // String | Filtro opcional por prefijo de título sin distinguir mayúsculas. (opcional)
let sortBy = PagesSortBy() // PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). (opcional)
let hasComments = true // Bool | Si es true, devolver únicamente páginas con al menos un comentario. (opcional)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---