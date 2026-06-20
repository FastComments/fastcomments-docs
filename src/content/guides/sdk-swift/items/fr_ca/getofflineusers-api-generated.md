Les commentateurs précédents sur la page qui NE sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Membres".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} à partir de afterName vers l'avant via $gt, sans coût de $skip.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départage du curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les entrées en cas d'égalité de noms ne soient pas perdues. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont toujours en version bêta. Pour tout problème, veuillez signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant de l'URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : transmettre nextAfterName de la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Départage du curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les entrées en cas d'égalité de noms ne soient pas perdues. (optionnel)

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