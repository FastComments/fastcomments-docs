Commentateurs passés sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
À utiliser après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
depuis afterName en avant via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les ex æquo de nom ne suppriment pas d'entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant de l'URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : passez nextAfterName de la réponse précédente. (optional)
let afterUserId = "afterUserId_example" // String | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les ex æquo de nom ne suppriment pas d'entrées. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]