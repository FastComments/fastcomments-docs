## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| usernameStartsWith | string | query | Όχι |  |
| mentionGroupIds | array | query | Όχι |  |
| sso | string | query | Όχι |  |
| searchSection | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (προαιρετικό)
let mentionGroupIds = ["inner_example"] // [String] |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)
let searchSection = "searchSection_example" // String |  (προαιρετικό)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]