## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| order | string | query | いいえ |  |
| after | number | query | いいえ |  |
| before | number | query | いいえ |  |

## レスポンス

戻り値: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  （オプション）
let skip = 987 // Double |  （オプション）
let order = SORT_DIR() // SORTDIR |  （オプション）
let after = 987 // Double |  （オプション）
let before = 987 // Double |  （オプション）

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]