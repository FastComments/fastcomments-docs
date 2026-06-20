Liste les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Exige que `enableFChat` soit true dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant le SSO sont filtrées en fonction de l'accès de groupe de l'utilisateur demandeur.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, 50 par défaut |
| q | string | query | No | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (les plus commentées en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si true, ne renvoie que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Curseur de pagination opaque renvoyé comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. (optionnel)
let limit = 987 // Int | 1..200, 50 par défaut (optionnel)
let q = "q_example" // String | Filtre optionnel de préfixe de titre insensible à la casse. (optionnel)
let sortBy = PagesSortBy() // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (les plus commentées en premier), ou `title` (alphabétique). (optionnel)
let hasComments = true // Bool | Si true, ne renvoyer que les pages ayant au moins un commentaire. (optionnel)

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