## パラメータ

| 名前 | Type | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## レスポンス

戻り値: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new に報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (任意)
let limit = 987 // Int |  (任意)
let skip = 987 // Int |  (任意)
let asTree = true // Bool |  (任意)
let skipChildren = 987 // Int |  (任意)
let limitChildren = 987 // Int |  (任意)
let maxTreeDepth = 987 // Int |  (任意)
let urlId = "urlId_example" // String |  (任意)
let userId = "userId_example" // String |  (任意)
let anonUserId = "anonUserId_example" // String |  (任意)
let contextUserId = "contextUserId_example" // String |  (任意)
let hashTag = "hashTag_example" // String |  (任意)
let parentId = "parentId_example" // String |  (任意)
let direction = SortDirections() // SortDirections |  (任意)
let fromDate = 987 // Int64 |  (任意)
let toDate = 987 // Int64 |  (任意)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]