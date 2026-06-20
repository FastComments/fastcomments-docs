---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (neobvezno)
let byIPFromComment = "byIPFromComment_example" // String |  (neobvezno)
let filters = "filters_example" // String |  (neobvezno)
let searchFilters = "searchFilters_example" // String |  (neobvezno)
let sorts = "sorts_example" // String |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

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