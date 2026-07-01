List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor opaco de paginación devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, predeterminado 50 |
| q | string | query | No | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (predeterminado, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es verdadero, solo devuelve páginas con al menos un comentario. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'Ejemplo getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código todavía están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor opaco de paginación devuelto como `nextCursor` de una solicitud anterior. Vinculado al mismo `sortBy`. (optional)
let limit = 987 // Int | 1..200, predeterminado 50 (optional)
let q = "q_example" // String | Filtro opcional de prefijo de título que no distingue entre mayúsculas y minúsculas. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Orden de clasificación. `updatedAt` (predeterminado, más reciente primero), `commentCount` (más comentarios primero), o `title` (alfabético). (optional)
let hasComments = true // Bool | Si es verdadero, solo devuelve páginas con al menos un comentario. (optional)

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