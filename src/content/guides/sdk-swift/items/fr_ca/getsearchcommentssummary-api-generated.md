## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| value | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchCommentsSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (optionnel)
let filters = "filters_example" // String |  (optionnel)
let searchFilters = "searchFilters_example" // String |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

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