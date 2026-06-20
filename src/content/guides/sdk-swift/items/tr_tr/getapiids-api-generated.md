---
## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Hayır |  |
| byIPFromComment | string | query | Hayır |  |
| filters | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| afterId | string | query | Hayır |  |
| demo | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Dönüş: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getApiIds Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (isteğe bağlı)
let byIPFromComment = "byIPFromComment_example" // String |  (isteğe bağlı)
let filters = "filters_example" // String |  (isteğe bağlı)
let searchFilters = "searchFilters_example" // String |  (isteğe bağlı)
let afterId = "afterId_example" // String |  (isteğe bağlı)
let demo = true // Bool |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

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

---