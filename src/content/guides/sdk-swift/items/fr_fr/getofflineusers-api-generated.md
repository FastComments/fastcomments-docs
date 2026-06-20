Anciens commentateurs de la page qui ne sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Membres".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
index à partir de afterName vers l'avant via $gt, sans coût lié à $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Tiebreaker du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la perte d'entrées. |

## Réponse

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant de l'URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : passez nextAfterName depuis la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Tiebreaker du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la perte d'entrées. (optionnel)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]