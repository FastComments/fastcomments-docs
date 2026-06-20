## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgeProgressListResponse.swift)

## 例

[inline-code-attrs-start title = 'getUserBadgeProgressList の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (オプション)
let limit = 987 // Double |  (オプション)
let skip = 987 // Double |  (オプション)

DefaultAPI.getUserBadgeProgressList(tenantId: tenantId, userId: userId, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]