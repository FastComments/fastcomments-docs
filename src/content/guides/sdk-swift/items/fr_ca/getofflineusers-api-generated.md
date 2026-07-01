Past commentateurs sur la page QUI NE sont PAS en ligne actuellement. Triés par displayName.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».  
Pagination par curseur sur commenterName : le serveur parcourt le fragment partiel {tenantId, urlId, commenterName} à partir de afterName vers l’avant via $gt, sans coût $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d’URL de page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettez nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Critère de rupture d’égalité du curseur : transmettez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifiant d’URL de page (nettoyé côté serveur).
let afterName = "afterName_example" // String | Curseur : transmettez nextAfterName de la réponse précédente. (optionnel)
let afterUserId = "afterUserId_example" // String | Critère de rupture d’égalité du curseur : transmettez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. (optionnel)

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