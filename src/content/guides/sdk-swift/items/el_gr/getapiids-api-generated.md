## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Όχι |  |
| byIPFromComment | string | query | Όχι |  |
| filters | string | query | Όχι |  |
| searchFilters | string | query | Όχι |  |
| afterId | string | query | Όχι |  |
| demo | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getApiIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (προαιρετικό)
let byIPFromComment = "byIPFromComment_example" // String |  (προαιρετικό)
let filters = "filters_example" // String |  (προαιρετικό)
let searchFilters = "searchFilters_example" // String |  (προαιρετικό)
let afterId = "afterId_example" // String |  (προαιρετικό)
let demo = true // Bool |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.getApiIds(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]