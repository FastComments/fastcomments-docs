Currently‑online viewers of a page: people whose websocket session is subscribed to the page right now.  
Renvoie anonCount + totalCount (room‑wide subscribers, including anon viewers we don't enumerate).

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d’URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Critère de désambigüisation du curseur : transmettre nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de noms ne suppriment pas d’entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant d’URL de la page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : transmettre nextAfterName de la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Critère de désambigüisation du curseur : transmettre nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de noms ne suppriment pas d’entrées. (optionnel)

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