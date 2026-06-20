## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| afterId | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getApiIds 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍屬測試階段。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (可選)
let byIPFromComment = "byIPFromComment_example" // String |  (可選)
let filters = "filters_example" // String |  (可選)
let searchFilters = "searchFilters_example" // String |  (可選)
let afterId = "afterId_example" // String |  (可選)
let demo = true // Bool |  (可選)
let sso = "sso_example" // String |  (可選)

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