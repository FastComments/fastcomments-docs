## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Ні |  |
| count | number | query | Ні |  |
| text-search | string | query | Ні |  |
| byIPFromComment | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| sorts | string | query | Ні |  |
| demo | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getApiComments Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще в бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (необов'язково)
let count = 987 // Double |  (необов'язково)
let textSearch = "textSearch_example" // String |  (необов'язково)
let byIPFromComment = "byIPFromComment_example" // String |  (необов'язково)
let filters = "filters_example" // String |  (необов'язково)
let searchFilters = "searchFilters_example" // String |  (необов'язково)
let sorts = "sorts_example" // String |  (необов'язково)
let demo = true // Bool |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getApiComments(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso) { (response, error) in
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