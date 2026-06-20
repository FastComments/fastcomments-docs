## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Όχι |  |
| filters | string | query | Όχι |  |
| searchFilters | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSearchCommentsSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (προαιρετικό)
let filters = "filters_example" // String |  (προαιρετικό)
let searchFilters = "searchFilters_example" // String |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]