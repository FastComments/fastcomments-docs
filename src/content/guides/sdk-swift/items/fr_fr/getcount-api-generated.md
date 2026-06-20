## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filter | string | query | Non |  |
| searchFilters | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (optionnel)
let byIPFromComment = "byIPFromComment_example" // String |  (optionnel)
let filter = "filter_example" // String |  (optionnel)
let searchFilters = "searchFilters_example" // String |  (optionnel)
let demo = true // Bool |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

ModerationAPI.getCount(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]