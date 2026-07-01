## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| page | number | query | Ne |  |
| count | number | query | Ne |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći uzorci koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double |  (opcionalno)
let count = 987 // Double |  (opcionalno)
let textSearch = "textSearch_example" // String |  (opcionalno)
let byIPFromComment = "byIPFromComment_example" // String |  (opcionalno)
let filters = "filters_example" // String |  (opcionalno)
let searchFilters = "searchFilters_example" // String |  (opcionalno)
let sorts = "sorts_example" // String |  (opcionalno)
let demo = true // Bool |  (opcionalno)
let sso = "sso_example" // String |  (opcionalno)

ModerationAPI.getApiComments(tenantId: tenantId, options: ModerationAPI.GetApiCommentsOptions(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]