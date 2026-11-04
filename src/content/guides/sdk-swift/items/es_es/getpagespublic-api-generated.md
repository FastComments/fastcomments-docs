Lista las páginas de un tenant. Lo utiliza el cliente de escritorio FChat para completar su lista de salas.
Requiere que `enableFChat` sea true en la configuración personalizada resuelta de cada página.
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario solicitante.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional de prefijo de título sin distinción de mayúsculas y minúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (con más comentarios primero) o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. (optional)
let limit = 987 // Int | 1..200, por defecto 50 (optional)
let q = "q_example" // String | Filtro opcional de prefijo de título sin distinción de mayúsculas y minúsculas. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (con más comentarios primero) o `title` (alfabético). (optional)
let hasComments = true // Bool | Si es true, solo devuelve páginas con al menos un comentario. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]