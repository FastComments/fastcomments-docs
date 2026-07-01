List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` depuis une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, valeur par défaut 50 |
| q | string | query | No | Filtre de préfixe de titre insensible à la casse, optionnel. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne renvoie que les pages contenant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let cursor = "cursor_example" // String | Curseur de pagination opaque renvoyé comme `nextCursor` depuis une requête précédente. Lié au même `sortBy`. (optionnel)
let limit = 987 // Int | 1..200, valeur par défaut 50 (optionnel)
let q = "q_example" // String | Filtre de préfixe de titre insensible à la casse, optionnel. (optionnel)
let sortBy = PagesSortBy() // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). (optionnel)
let hasComments = true // Bool | Si vrai, ne renvoie que les pages contenant au moins un commentaire. (optionnel)

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