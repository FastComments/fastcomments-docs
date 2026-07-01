List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | No | 1..200, padrão 50 |
| q | string | query | No | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. |
| sortBy | string | query | No | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabético). |
| hasComments | boolean | query | No | Se true, retornar apenas páginas com pelo menos um comentário. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda são beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. (optional)
let limit = 987 // Int | 1..200, padrão 50 (optional)
let q = "q_example" // String | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabético). (optional)
let hasComments = true // Bool | Se true, retornar apenas páginas com pelo menos um comentário. (optional)

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