## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| text-search | string | query | Όχι |  |
| byIPFromComment | string | query | Όχι |  |
| filters | string | query | Όχι |  |
| searchFilters | string | query | Όχι |  |
| sorts | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (προαιρετικό)
let byIPFromComment = "byIPFromComment_example" // String |  (προαιρετικό)
let filters = "filters_example" // String |  (προαιρετικό)
let searchFilters = "searchFilters_example" // String |  (προαιρετικό)
let sorts = "sorts_example" // String |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.postApiExport(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso) { (response, error) in
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