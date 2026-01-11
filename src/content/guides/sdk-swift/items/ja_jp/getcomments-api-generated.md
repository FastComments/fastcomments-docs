## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| skip | integer | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |
| contextUserId | string | query | いいえ |  |
| hashTag | string | query | いいえ |  |
| parentId | string | query | いいえ |  |
| direction | string | query | いいえ |  |

## レスポンス

戻り値: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータです。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (オプション)
let limit = 987 // Int |  (オプション)
let skip = 987 // Int |  (オプション)
let asTree = true // Bool |  (オプション)
let skipChildren = 987 // Int |  (オプション)
let limitChildren = 987 // Int |  (オプション)
let maxTreeDepth = 987 // Int |  (オプション)
let urlId = "urlId_example" // String |  (オプション)
let userId = "userId_example" // String |  (オプション)
let anonUserId = "anonUserId_example" // String |  (オプション)
let contextUserId = "contextUserId_example" // String |  (オプション)
let hashTag = "hashTag_example" // String |  (オプション)
let parentId = "parentId_example" // String |  (オプション)
let direction = SortDirections() // SortDirections |  (オプション)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction) { (response, error) in
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