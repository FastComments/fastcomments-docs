## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'resetUserNotifications Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (προαιρετικό)
let afterCreatedAt = 987 // Int64 |  (προαιρετικό)
let unreadOnly = true // Bool |  (προαιρετικό)
let dmOnly = true // Bool |  (προαιρετικό)
let noDm = true // Bool |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

PublicAPI.resetUserNotifications(tenantId: tenantId, options: PublicAPI.ResetUserNotificationsOptions(afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]