## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filter | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta. Bitte melden Sie Probleme unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (optional)
let byIPFromComment = "byIPFromComment_example" // String |  (optional)
let filter = "filter_example" // String |  (optional)
let searchFilters = "searchFilters_example" // String |  (optional)
let demo = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

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

---