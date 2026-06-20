## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| page | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| skip | integer | query | 否 |  |
| asTree | boolean | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |
| contextUserId | string | query | 否 |  |
| hashTag | string | query | 否 |  |
| parentId | string | query | 否 |  |
| direction | string | query | 否 |  |
| fromDate | integer | query | 否 |  |
| toDate | integer | query | 否 |  |

## 回應

回傳: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (選用)
let limit = 987 // Int |  (選用)
let skip = 987 // Int |  (選用)
let asTree = true // Bool |  (選用)
let skipChildren = 987 // Int |  (選用)
let limitChildren = 987 // Int |  (選用)
let maxTreeDepth = 987 // Int |  (選用)
let urlId = "urlId_example" // String |  (選用)
let userId = "userId_example" // String |  (選用)
let anonUserId = "anonUserId_example" // String |  (選用)
let contextUserId = "contextUserId_example" // String |  (選用)
let hashTag = "hashTag_example" // String |  (選用)
let parentId = "parentId_example" // String |  (選用)
let direction = SortDirections() // SortDirections |  (選用)
let fromDate = 987 // Int64 |  (選用)
let toDate = 987 // Int64 |  (選用)

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