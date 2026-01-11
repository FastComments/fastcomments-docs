## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| badgeId | string | query | Όχι |  |
| type | number | query | Όχι |  |
| displayedOnComments | boolean | query | Όχι |  |
| limit | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (προαιρετικό)
let badgeId = "badgeId_example" // String |  (προαιρετικό)
let type = 987 // Double |  (προαιρετικό)
let displayedOnComments = true // Bool |  (προαιρετικό)
let limit = 987 // Double |  (προαιρετικό)
let skip = 987 // Double |  (προαιρετικό)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]