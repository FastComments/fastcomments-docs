## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sorts | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## 例

[inline-code-attrs-start title = 'postApiExport の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (任意)
let byIPFromComment = "byIPFromComment_example" // String |  (任意)
let filters = "filters_example" // String |  (任意)
let searchFilters = "searchFilters_example" // String |  (任意)
let sorts = "sorts_example" // String |  (任意)
let sso = "sso_example" // String |  (任意)

ModerationAPI.postApiExport(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso) { (response, error) in
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