## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| page | number | query | いいえ |  |
| count | number | query | いいえ |  |
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sorts | string | query | いいえ |  |
| demo | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## 例

[inline-code-attrs-start title = 'getApiComments の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let page = 987 // Double |  (オプション)
let count = 987 // Double |  (オプション)
let textSearch = "textSearch_example" // String |  (オプション)
let byIPFromComment = "byIPFromComment_example" // String |  (オプション)
let filters = "filters_example" // String |  (オプション)
let searchFilters = "searchFilters_example" // String |  (オプション)
let sorts = "sorts_example" // String |  (オプション)
let demo = true // Bool |  (オプション)
let sso = "sso_example" // String |  (オプション)

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