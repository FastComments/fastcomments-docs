Actuellement, les spectateurs en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés à la salle, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passer nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départageur de curseur : passer nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini pour que les égalités de noms n'entraînent pas la suppression d'entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant de l'URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : passer nextAfterName de la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Départageur de curseur : passer nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini pour que les égalités de noms n'entraînent pas la suppression d'entrées. (optionnel)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]