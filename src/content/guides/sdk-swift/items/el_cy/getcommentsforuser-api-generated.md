## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Απόκριση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Το παρακάτω δείγμα κώδικα βρίσκεται ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε να το αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (προαιρετικό)
let direction = SortDirections() // SortDirections |  (προαιρετικό)
let repliesToUserId = "repliesToUserId_example" // String |  (προαιρετικό)
let page = 987 // Double |  (προαιρετικό)
let includei10n = true // Bool |  (προαιρετικό)
let locale = "locale_example" // String |  (προαιρετικό)
let isCrawler = true // Bool |  (προαιρετικό)

PublicAPI.getCommentsForUser(options: PublicAPI.GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]