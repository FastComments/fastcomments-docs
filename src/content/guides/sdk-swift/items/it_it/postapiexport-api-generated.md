## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Esempio

[inline-code-attrs-start title = 'postApiExport Esempio'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Il seguente esempio di codice è ancora in beta. Per qualsiasi problema, si prega di segnalarlo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  (opzionale)
let byIPFromComment = "byIPFromComment_example" // String |  (opzionale)
let filters = "filters_example" // String |  (opzionale)
let searchFilters = "searchFilters_example" // String |  (opzionale)
let sorts = "sorts_example" // String |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

ModerationAPI.postApiExport(tenantId: tenantId, options: ModerationAPI.PostApiExportOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]