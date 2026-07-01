## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## レスポンス

返却: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## 例

[inline-code-attrs-start title = 'getApiComments 例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new に報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double |  (オプション)
let count = 987 // Double |  (オプション)
let textSearch = "textSearch_example" // String |  (オプション)
let byIPFromComment = "byIPFromComment_example" // String |  (オプション)
let filters = "filters_example" // String |  (オプション)
let searchFilters = "searchFilters_example" // String |  (オプション)
let sorts = "sorts_example" // String |  (オプション)
let demo = true // Bool |  (オプション)
let sso = "sso_example" // String |  (オプション)

ModerationAPI.getApiComments(tenantId: tenantId, options: ModerationAPI.GetApiCommentsOptions(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]