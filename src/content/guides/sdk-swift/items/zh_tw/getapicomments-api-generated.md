## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getApiComments 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let page = 987 // Double |  (選用)
let count = 987 // Double |  (選用)
let textSearch = "textSearch_example" // String |  (選用)
let byIPFromComment = "byIPFromComment_example" // String |  (選用)
let filters = "filters_example" // String |  (選用)
let searchFilters = "searchFilters_example" // String |  (選用)
let sorts = "sorts_example" // String |  (選用)
let demo = true // Bool |  (選用)
let sso = "sso_example" // String |  (選用)

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