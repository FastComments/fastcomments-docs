---
## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'postApiExport Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du opdager et problem, bedes du rapportere det via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (valgfri)
let byIPFromComment = "byIPFromComment_example" // String |  (valgfri)
let filters = "filters_example" // String |  (valgfri)
let searchFilters = "searchFilters_example" // String |  (valgfri)
let sorts = "sorts_example" // String |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

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