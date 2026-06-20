## 参数

| Name | Type | Location | Required | Description |
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

## 响应

返回: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getComments 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (可选)
let limit = 987 // Int |  (可选)
let skip = 987 // Int |  (可选)
let asTree = true // Bool |  (可选)
let skipChildren = 987 // Int |  (可选)
let limitChildren = 987 // Int |  (可选)
let maxTreeDepth = 987 // Int |  (可选)
let urlId = "urlId_example" // String |  (可选)
let userId = "userId_example" // String |  (可选)
let anonUserId = "anonUserId_example" // String |  (可选)
let contextUserId = "contextUserId_example" // String |  (可选)
let hashTag = "hashTag_example" // String |  (可选)
let parentId = "parentId_example" // String |  (可选)
let direction = SortDirections() // SortDirections |  (可选)
let fromDate = 987 // Int64 |  (可选)
let toDate = 987 // Int64 |  (可选)

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

---