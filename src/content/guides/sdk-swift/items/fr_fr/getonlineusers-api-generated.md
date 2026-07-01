Visionneurs en ligne d'une page : personnes dont la session WebSocket est abonnée à la page en ce moment. Retourne anonCount + totalCount (abonnés de la salle, y compris les visiteurs anonymes que nous n'énumérotions pas).

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettre nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Anti-égalité du curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées. |

## Réponse

Retourne : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les échantillons de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant d'URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : transmettre nextAfterName depuis la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Anti-égalité du curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées. (optionnel)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]