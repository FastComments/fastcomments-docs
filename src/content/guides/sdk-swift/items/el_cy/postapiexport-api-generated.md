## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
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