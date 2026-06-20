## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| newStatus | string | path | Ja |  |
| sso | string | query | Nein |  |

## Response

Gibt zurück: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatusResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationStatus Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let newStatus = "newStatus_example" // String | 
let sso = "sso_example" // String |  (optional)

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

---