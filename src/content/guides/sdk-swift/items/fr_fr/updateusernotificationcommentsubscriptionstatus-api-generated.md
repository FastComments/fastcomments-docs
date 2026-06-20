Activer ou désactiver les notifications pour un commentaire spécifique.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| notificationId | string | path | Oui |  |
| optedInOrOut | string | path | Oui |  |
| commentId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationCommentSubscriptionStatusResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateUserNotificationCommentSubscriptionStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let optedInOrOut = "optedInOrOut_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (optionnel)

PublicAPI.updateUserNotificationCommentSubscriptionStatus(tenantId: tenantId, notificationId: notificationId, optedInOrOut: optedInOrOut, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]