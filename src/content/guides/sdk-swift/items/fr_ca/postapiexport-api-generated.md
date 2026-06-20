## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (optionnel)
let byIPFromComment = "byIPFromComment_example" // String |  (optionnel)
let filters = "filters_example" // String |  (optionnel)
let searchFilters = "searchFilters_example" // String |  (optionnel)
let sorts = "sorts_example" // String |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

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