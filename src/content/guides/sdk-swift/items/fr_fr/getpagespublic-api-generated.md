Liste les pages d'un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit à true dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant un SSO sont filtrées selon les accès de groupe de l'utilisateur à l'origine de la requête.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé sous forme de `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, 50 par défaut |
| q | string | query | Non | Filtre facultatif insensible à la casse sur le préfixe du titre. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus grand nombre de commentaires d'abord) ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si true, ne renvoie que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Curseur de pagination opaque renvoyé sous forme de `nextCursor` par une requête précédente. Lié au même `sortBy`. (optional)
let limit = 987 // Int | 1..200, 50 par défaut (optional)
let q = "q_example" // String | Filtre facultatif insensible à la casse sur le préfixe du titre. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus grand nombre de commentaires d'abord) ou `title` (alphabétique). (optional)
let hasComments = true // Bool | Si true, ne renvoie que les pages ayant au moins un commentaire. (optional)

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