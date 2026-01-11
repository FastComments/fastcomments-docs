## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| afterId | string | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotifications200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα resetUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (προαιρετικό)
let afterCreatedAt = 987 // Int64 |  (προαιρετικό)
let unreadOnly = true // Bool |  (προαιρετικό)
let dmOnly = true // Bool |  (προαιρετικό)
let noDm = true // Bool |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

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