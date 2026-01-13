## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
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

## 响应

返回: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## 示例

[inline-code-attrs-start title = 'getComments 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
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