## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filters | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| afterId | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getApiIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (optional)
let byIPFromComment = "byIPFromComment_example" // String |  (optional)
let filters = "filters_example" // String |  (optional)
let searchFilters = "searchFilters_example" // String |  (optional)
let afterId = "afterId_example" // String |  (optional)
let demo = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

ModerationAPI.getApiIds(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]