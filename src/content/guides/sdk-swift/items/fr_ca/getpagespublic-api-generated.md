List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre de préfixe de titre insensible à la casse (facultatif). |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne renvoie que les pages contenant au moins un commentaire. |

## Response

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. (facultatif)
let limit = 987 // Int | 1..200, par défaut 50 (facultatif)
let q = "q_example" // String | Filtre de préfixe de titre insensible à la casse. (facultatif)
let sortBy = PagesSortBy() // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). (facultatif)
let hasComments = true // Bool | Si vrai, ne renvoie que les pages contenant au moins un commentaire. (facultatif)

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