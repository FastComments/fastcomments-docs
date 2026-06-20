## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| afterId | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getApiIds Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још увијек у бета фази. За сваки проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (опционо)
let byIPFromComment = "byIPFromComment_example" // String |  (опционо)
let filters = "filters_example" // String |  (опционо)
let searchFilters = "searchFilters_example" // String |  (опционо)
let afterId = "afterId_example" // String |  (опционо)
let demo = true // Bool |  (опционо)
let sso = "sso_example" // String |  (опционо)

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