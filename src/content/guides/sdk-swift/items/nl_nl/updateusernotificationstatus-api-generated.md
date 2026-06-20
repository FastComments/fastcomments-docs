## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| newStatus | string | path | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatusResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationStatus Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let newStatus = "newStatus_example" // String | 
let sso = "sso_example" // String |  (optioneel)

PublicAPI.updateUserNotificationStatus(tenantId: tenantId, notificationId: notificationId, newStatus: newStatus, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]