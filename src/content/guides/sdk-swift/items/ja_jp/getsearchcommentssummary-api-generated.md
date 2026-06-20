---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| value | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## 例

[inline-code-attrs-start title = 'getSearchCommentsSummary の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let value = "value_example" // String |  (任意)
let filters = "filters_example" // String |  (任意)
let searchFilters = "searchFilters_example" // String |  (任意)
let sso = "sso_example" // String |  (任意)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
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