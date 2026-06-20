Liste les pages pour un tenant. Utilisé par le client de bureau FChat pour alimenter sa liste des salons.
Requiert que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant SSO sont filtrées en fonction des droits de groupe de l'utilisateur demandeur.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé en tant que `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 |
| q | string | query | Non | Filtre optionnel par préfixe de titre insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus commenté en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si vrai, ne renvoyer que les pages comportant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Curseur de pagination opaque renvoyé en tant que `nextCursor` par une requête précédente. Lié au même `sortBy`. (optionnel)
let limit = 987 // Int | 1..200, par défaut 50 (optionnel)
let q = "q_example" // String | Filtre optionnel par préfixe de titre insensible à la casse. (optionnel)
let sortBy = PagesSortBy() // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus commenté en premier), ou `title` (alphabétique). (optionnel)
let hasComments = true // Bool | Si vrai, ne renvoyer que les pages comportant au moins un commentaire. (optionnel)

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