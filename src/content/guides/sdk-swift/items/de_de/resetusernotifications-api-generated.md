## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nein |  |
| afterCreatedAt | integer | query | Nein |  |
| unreadOnly | boolean | query | Nein |  |
| dmOnly | boolean | query | Nein |  |
| noDm | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für resetUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optional)
let afterCreatedAt = 987 // Int64 |  (optional)
let unreadOnly = true // Bool |  (optional)
let dmOnly = true // Bool |  (optional)
let noDm = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.resetUserNotifications(tenantId: tenantId, afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso) { (response, error) in
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